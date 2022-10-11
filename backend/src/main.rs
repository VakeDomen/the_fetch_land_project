use std::env;
use actix_cors::Cors;
use actix_web_httpauth::extractors::bearer::Config;
use api::{auth_hook::auth, auth_login::login, user_update::{user_update}, card_id::get_card, card_name::get_card_by_name, user_get::user_get, user_delete::user_delete, user_sales::user_sales, user_sale_delete::user_sale_delete, user_sale_new::user_sale_new, card_sales::card_sales, card_sales_name::{get_card_sales_by_name, get_card_sales_by_name_partials}, user_credentials::user_credentials, card_sales_latest::{card_sales_latest, card_sales_latest_default}, user_sale_edit::user_sale_edit, card_sales_paged::card_sales_paged, card_sales_num::card_sales_num, notification_telegram::notify_telegram};
use dotenv::dotenv;
use actix_web::{web::Data, App, HttpServer, http, middleware::Logger};
use models::state::AppState;
use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl, basic::BasicClient, RedirectUrl};
use services::card_cache::{setup_card_cache, ALL_CARDS};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::api::card_sales_id::get_card_sales_by_id;

mod models;
mod api;
mod database;
mod services;

#[macro_use] extern crate diesel;
extern crate env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // setup logging and env variables
    println!("[SETUP] Setting up environment.");
    let (
        port,
        own_ssl,
        cors_url_1,
        cors_url_2,
        auth_hook_url,
        oauth_client_id,
        oauth_client_secret,
    ) = setup_env();
    
    // setup card cache (may take a while if no cards loded yet
    // in its own block to drop the lock on ALL_CARDS after check
    let is_empty = {
        println!("[SETUP] Checking for cards on system...");
        let hm = ALL_CARDS.lock().unwrap();
        hm.is_empty()
    };
    if is_empty {
        println!("[SETUP] Cards not on system, fetching cards...");
        setup_card_cache().await;
    } else{
        println!("[SETUP] Cards already stored, skipping fetch.");
    }

    // setup Http server
    let mut server = HttpServer::new(move || {
        // setup CORS
        let cors = Cors::default()
            .allowed_origin(cors_url_1.as_str())
            .allowed_origin(cors_url_2.as_str())
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        // setup google auth client
        let google_client_id = ClientId::new(oauth_client_id.to_owned());
        let google_client_secret_struct = ClientSecret::new(oauth_client_secret.to_owned());
        let auth_url_endpoint = "https://accounts.google.com/o/oauth2/v2/auth".to_string();
        let token_url_endpoint = "https://www.googleapis.com/oauth2/v3/token".to_string();
        let redirect_url = auth_hook_url.to_string();
        let auth_url = AuthUrl::new(auth_url_endpoint).expect("Invalid authorization endpoint URL");
        let token_url_struct = TokenUrl::new(token_url_endpoint).expect("Invalid token endpoint URL");
        let token_url = Some(token_url_struct);
        let google_client_secret = Some(google_client_secret_struct);
        // Set up the config for the Google OAuth2 process.
        let client = BasicClient::new(
            google_client_id,
            google_client_secret,
            auth_url,
            token_url,
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect_url)
                .expect("Invalid redirect URL"),
        );

        // setup routes
        App::new()
            // .wrap(Logger::default())
            .wrap(Logger::new("TIME: %T s | FROM: %a | RESP: %s | %r %{User-Agent}i (msg size in byted: %b)"))
            .wrap(cors)
            .app_data(Data::new(AppState { oauth: client }))
            .app_data(Config::default())
            .service(auth)
            .service(login)
            .service(notify_telegram)
            .service(get_card)
            .service(get_card_by_name)
            .service(card_sales)
            .service(get_card_sales_by_name)
            .service(user_update)
            .service(user_get)
            .service(user_delete)
            .service(user_sales)
            .service(user_sale_new)
            .service(user_sale_delete)
            .service(user_credentials)
            .service(card_sales_latest_default)
            .service(card_sales_latest)
            .service(get_card_sales_by_name_partials)
            .service(get_card_sales_by_id)
            .service(user_sale_edit)
            .service(card_sales_paged)
            .service(card_sales_num)
    });
    if own_ssl {
        println!("[SETUP] Running server on 0.0.0.0:{} with own ssl", port);
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
        builder.set_certificate_chain_file("cert.pem").unwrap();
        server = server.bind_openssl(format!("0.0.0.0:{}", port), builder).unwrap();
    } else {
        println!("[SETUP] Running server on 0.0.0.0:{} without ssl", port);
        server = server.bind(("0.0.0.0", port)).unwrap();
    }
    server.run().await
}

fn setup_env() -> (u16, bool, String, String, String, String, String) {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let port = env::var("PORT").expect("$PORT is not set").parse::<u16>().unwrap();
    let own_ssl = match std::env::var("OWN_SSL") {
        Ok(val) => match val.as_str() {
            "true" => true,
            _ => false,
        },
        _ => false,
    };
    match std::env::var("DEBUG") {
        Ok(val) => {
            if val.eq("1") {
                println!("[SETUP] Logging set to debug!");
                std::env::set_var("RUST_LOG", "debug");
                std::env::set_var("RUST_BACKTRACE", "1");
            }
        },
        Err(_) => ()
    }
    env::var("TELOXIDE_TOKEN").expect("$TELOXIDE_TOKEN is not set");
    env::var("AUTH_REDIRECT_URL").expect("$AUTH_REDIRECT_URL is not set");
    let oauth_client_id = env::var("OAUTH_CLIENT_ID").expect("Missing the OAUTH_CLIENT_ID environment variable.");
    let oauth_client_secret = env::var("OAUTH_CLIENT_SECRET").expect("Missing the OAUTH_CLIENT_SECRET environment variable.");
    let cors_url_1 = env::var("CORS_DOMAIN_1").expect("Missing the CORS_DOMAIN_2 environment variable.");
    let cors_url_2 = env::var("CORS_DOMAIN_2").expect("Missing the CORS_DOMAIN_2 environment variable.");
    let auth_hook_url = env::var("AUTH_HOOK_URL").expect("Missing the AUTH_HOOK_URL environment variable.");
    (
        port,
        own_ssl,
        cors_url_1,
        cors_url_2,
        auth_hook_url,
        oauth_client_id,
        oauth_client_secret,
    )
}