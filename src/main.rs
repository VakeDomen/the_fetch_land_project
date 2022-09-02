use std::{fs::File, path::Path, io::Write, collections::HashMap};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use models::card::Card;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::models::{scryfall::{BulkData, BulkResponse}, trie::TrieTree, card::CardPublic};
mod models;


// const ALL_CARDS_HASH_MAP_FILE_PATH: &str = "./small_hm.json";
const ALL_CARDS_HASH_MAP_FILE_PATH: &str = "./all_cards_hash_map.json";
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

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_card)
            .service(get_card_by_name)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
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
    let path = Path::new(ALL_CARDS_FILE_PATH);
    let content = match response.text().await {
        Ok(c) => c,
        Err(e) => {
            println!("{:#?}", e.to_string());
            return;
        }
    };
    println!("Pasring cards");
    save_cards_to_cache(&content);
    println!("Writing cards to file");
    save_cache();
}

fn save_cards_to_cache(content: &String) -> () {
    let mut all_cards = ALL_CARDS.lock().unwrap();
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