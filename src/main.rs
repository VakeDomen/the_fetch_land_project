use std::env;
use actix_web_httpauth::extractors::bearer::Config;
use api::{auth_hook::auth, auth_login::login, user_update::{user_update}, card_id::get_card, card_name::get_card_by_name, user_get::user_get};
use dotenv::dotenv;
use actix_web::{web::Data, App, HttpServer};
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
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    dotenv().ok();
    // setup ssl
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    setup_card_cache().await;

    HttpServer::new(|| {
        let google_client_id = ClientId::new(
            env::var("OAUTH_CLIENT_ID")
                .expect("Missing the OAUTH_CLIENT_ID environment variable."),
        );
        let google_client_secret = ClientSecret::new(
            env::var("OAUTH_CLIENT_SECRET")
                .expect("Missing the OAUTH_CLIENT_SECRET environment variable."),
        );
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
            .expect("Invalid token endpoint URL");

        // Set up the config for the Google OAuth2 process.
        let client = BasicClient::new(
            google_client_id,
            Some(google_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(
            RedirectUrl::new("https://localhost:8080/auth/hook".to_string())
                .expect("Invalid redirect URL"),
        );

        App::new()
            .app_data(Data::new(AppState { oauth: client }))
            .app_data(Config::default())
            .service(auth)
            .service(get_card)
            .service(login)
            .service(get_card_by_name)
            .service(user_update)
            .service(user_get)
    })
    .bind_openssl("0.0.0.0:8080", builder)?
    .run()
    .await
}