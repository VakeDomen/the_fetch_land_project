use std::{collections::HashMap, env};
use api::{auth::auth, login::login};
use dotenv::dotenv;
use actix_web::{get, post, web::{self, Data}, App, HttpResponse, HttpServer, Responder};
use models::{card::Card, state::AppState};
use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl, basic::BasicClient, RedirectUrl};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::models::{scryfall::BulkResponse, trie::TrieTree, card::CardPublic};

mod models;
mod api;
mod database;
mod services;

#[macro_use] extern crate diesel;

const ALL_CARDS_HASH_MAP_FILE_PATH: &str = "./small_hm.json";
// const ALL_CARDS_HASH_MAP_FILE_PATH: &str = "./all_cards_hash_map.json";
const ALL_CARDS_FILE_PATH: &str = "./all_cards.json";

static ALL_CARDS: Lazy<Mutex<HashMap<String, Card>>> = Lazy::new(|| {
    match serde_any::from_file(ALL_CARDS_HASH_MAP_FILE_PATH) {
        Ok(hm) => Mutex::new(hm),
        Err(_) => Mutex::new(HashMap::new())
    }
});

static NAME_TRIE: Lazy<Mutex<TrieTree>> = Lazy::new(|| {
    let all_cards = ALL_CARDS.lock().unwrap();
    let mut trie = TrieTree::new();
    for card in all_cards.values().into_iter() {
        trie.insert(card.name.clone().to_lowercase(), card.id.clone());
    }
    Mutex::new(trie)
});

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

    download_card_data(&get_bulk_data().await.unwrap()).await;
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
            RedirectUrl::new("https://localhost:8080/auth".to_string())
                .expect("Invalid redirect URL"),
        );

        App::new()
            .app_data(Data::new(AppState { oauth: client }))
            .service(auth)
            .service(get_card)
            .service(login)
            .service(get_card_by_name)
    })
    .bind_openssl("0.0.0.0:8080", builder)?
    .run()
    .await
}

#[get("/c/{name}")]
async fn get_card(name: web::Path<String>) -> impl Responder {
    let all_cards = ALL_CARDS.lock().unwrap();
    let card = all_cards.get(&name.to_string());
    match card {
        Some(c) => HttpResponse::Ok().json(CardPublic::from(c)),
        None => HttpResponse::NotFound().finish()
    }
}

#[get("/n/{name}")]
async fn get_card_by_name(name: web::Path<String>) -> impl Responder {
    let trie = NAME_TRIE.lock().unwrap();
    let card_ids = trie.collect(name.to_string().to_lowercase());
    let all_cards = ALL_CARDS.lock().unwrap();
    let cards: Vec<&Card> = card_ids.into_iter().map(|id| all_cards.get(&id).unwrap()).collect();
    let pub_cards: Vec<CardPublic> = cards.iter().map(|c| CardPublic::from(c)).collect();
    HttpResponse::Ok().json(pub_cards)
}

async fn download_card_data(bulk_response: &BulkResponse) {
    let mut card_uri = None;
    for bulk_data in bulk_response.data.iter() {
        if bulk_data.data_type.eq("all_cards") {
            card_uri = Some(bulk_data.download_uri.clone());
        }
    }
    let target = match card_uri {
        Some(u) => u,
        None => return,
    };
    println!("Dowanloading cards");
    let response = match reqwest::get(target).await {
        Ok(resp) => resp,
        Err(e) => {
            println!("{:#?}", e.to_string());
            return;
        }
    };
    println!("Creating card file");
    let content = fetch_cards_data_text(response).await;
    println!("Pasring cards");
    match content {
        Some(c) => {
            save_cards_to_cache(&c);
            println!("Writing cards to file");
            save_cache();
        },
        None => {
            println!("Not refreshing cards");
        }
    }
    
}

async fn fetch_cards_data_text(response: reqwest::Response) -> Option<String> {
    let all_cards = ALL_CARDS.lock().unwrap();
    if all_cards.is_empty() {
        return match response.text().await {
            Ok(c) => Some(c),
            Err(e) => {
                println!("{:#?}", e.to_string());
                return None
            }
        };
    }
    None
}

fn save_cards_to_cache(content: &String) -> () {
    let mut all_cards = ALL_CARDS.lock().unwrap();
    if all_cards.is_empty() {
        let cards: Vec<Card> = match serde_json::from_str(content.as_str()) {
            Ok(cards) => cards,
            Err(e) => {
                println!("{}", e.to_string());
                return;
            },
        };
        println!("Parsed cards from string!");
        all_cards.clear();
        for card in cards.into_iter() {
            all_cards.insert(card.id.clone(), card);
        }
    }
}

fn save_cache() {
    let all_cards = ALL_CARDS.lock().unwrap();
    match serde_any::to_file(ALL_CARDS_HASH_MAP_FILE_PATH, &*all_cards) {
        Ok(_) => {},
        Err(e) => {println!("Error saving task queue: {:?}", e);}
    };
}

async fn get_bulk_data() -> Option<BulkResponse> {
    let resp = match reqwest::get("https://api.scryfall.com/bulk-data").await {
        Ok(data) => data,
        Err(e) =>  {
            println!("{:#?}", e.to_string());
            return None;
        },
    };
    let text = match resp.text().await {
        Ok(s) => s,
        Err(e) =>  {
            println!("{:#?}", e.to_string());
            return None;
        },
    };
    match serde_json::from_str(text.as_str()) {
        Ok(w) => Some(w),
        Err(e) => {
            println!("{:#?}", e.to_string());
            None
        },
    } 
}