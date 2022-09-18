use std::env;
use actix_cors::Cors;
use actix_web_httpauth::extractors::bearer::Config;
use api::{auth_hook::auth, auth_login::login, user_update::{user_update}, card_id::get_card, card_name::get_card_by_name, user_get::user_get, user_delete::user_delete, user_sales::user_sales, user_sale_delete::user_sale_delete, user_sale_new::user_sale_new, card_sales::card_sales, card_sales_name::{get_card_sales_by_name, get_card_sales_by_name_partials}, user_credentials::user_credentials, card_sales_latest::{card_sales_latest, card_sales_latest_default}, user_sale_edit::user_sale_edit, card_sales_paged::card_sales_paged, card_sales_num::card_sales_num};
use dotenv::dotenv;
use actix_web::{web::Data, App, HttpServer, http};
use models::state::AppState;
use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl, basic::BasicClient, RedirectUrl};
use services::card_cache::setup_card_cache;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod models;
mod api;
mod database;
mod services;

#[macro_use] extern crate diesel;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // setup logging and env variables
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    dotenv().ok();
    
    // setup ssl
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("cert.pem")
        .unwrap();

    // setup card cache (may take a while if no cards loded yet)
    setup_card_cache().await;

    // setup Http server
    HttpServer::new(|| {
        // setup CORS
        let cors = Cors::default()
              .allowed_origin("http://localhost:4200")
              .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);

        // setup google auth client
        let google_client_id = ClientId::new(
            env::var("OAUTH_CLIENT_ID")
                .expect("Missing the OAUTH_CLIENT_ID environment variable."),
        );
        let google_client_secret_struct = ClientSecret::new(
            env::var("OAUTH_CLIENT_SECRET")
                .expect("Missing the OAUTH_CLIENT_SECRET environment variable."),
        );
        let auth_url_endpoint = "https://accounts.google.com/o/oauth2/v2/auth".to_string();
        let token_url_endpoint = "https://www.googleapis.com/oauth2/v3/token".to_string();
        let redirect_url = "https://localhost:8080/auth/hook".to_string();
        
        let auth_url = AuthUrl::new(auth_url_endpoint)
            .expect("Invalid authorization endpoint URL");
        let token_url_struct = TokenUrl::new(token_url_endpoint)
            .expect("Invalid token endpoint URL");

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
            .wrap(cors)
            .app_data(Data::new(AppState { oauth: client }))
            .app_data(Config::default())
            .service(auth)
            .service(login)
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
            .service(user_sale_edit)
            .service(card_sales_paged)
            .service(card_sales_num)
    })
    .bind_openssl("0.0.0.0:8080", builder)?
    .run()
    .await
}