use actix_web::{get, web, Responder, HttpResponse};

use crate::{services::card_cache::{NAME_TRIE, ALL_CARDS}, models::card::{Card, CardPublic}};

#[get("/card/name/{lang}/{name}")]
async fn get_card_by_name(params: web::Path<(String, String)>) -> impl Responder {
    let (lang, name) = params.into_inner();
    let card_ids = {
        let trie = NAME_TRIE.lock().unwrap();
        trie.collect(name.to_lowercase())
    };
    let all_cards = ALL_CARDS.lock().unwrap();
    let cards: Vec<&Card> = card_ids
        .into_iter()
        .map(|id| all_cards.get(&id).unwrap())
        .filter(|c| c.lang.eq(&lang.to_string()))
        .collect();
    let pub_cards: Vec<CardPublic> = cards
        .iter()
        .map(|c| CardPublic::from(c))
        .collect();
    HttpResponse::Ok().json(pub_cards)
}