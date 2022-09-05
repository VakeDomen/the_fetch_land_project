use actix_web::{get, web, Responder, HttpResponse};

use crate::{services::card_cache::{NAME_TRIE, ALL_CARDS}, models::card::{Card, CardPublic}};

#[get("/card/name/{name}")]
async fn get_card_by_name(name: web::Path<String>) -> impl Responder {
    let trie = NAME_TRIE.lock().unwrap();
    let card_ids = trie.collect(name.to_string().to_lowercase());
    let all_cards = ALL_CARDS.lock().unwrap();
    let cards: Vec<&Card> = card_ids.into_iter().map(|id| all_cards.get(&id).unwrap()).collect();
    let pub_cards: Vec<CardPublic> = cards.iter().map(|c| CardPublic::from(c)).collect();
    HttpResponse::Ok().json(pub_cards)
}