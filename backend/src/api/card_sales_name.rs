use actix_web::{get, web, Responder, HttpResponse};

use crate::{services::{card_cache::{NAME_TRIE, ALL_CARDS, NAME_PARTIALS_TRIE}, database::sale_operations::get_sales_by_card}, models::sale::Sale};

#[get("/card/sales/name/{lang}/{name}")]
async fn get_card_sales_by_name(params: web::Path<(String, String)>) -> impl Responder {
    let (lang, name) = params.into_inner();
    let mut card_ids = {
        let trie = NAME_TRIE.lock().unwrap();
        trie.collect(name.to_lowercase())
    };
    let filtered_cards_ids: Vec<String> = {
        let all_cards = ALL_CARDS.lock().unwrap();
        card_ids.dedup();
        card_ids
            .into_iter()
            .map(|id| all_cards.get(&id).unwrap())
            .filter(|c| c.lang.eq(&lang.to_string()))
            .map(|card| card.id.clone())
            .collect()
    };
    let sales: Vec<Sale>= filtered_cards_ids
        .iter()
        .filter_map(|c| get_sales_by_card(c.to_string()).ok())
        .flatten()
        .map(Sale::from)
        .collect();        
    HttpResponse::Ok().json(sales)
}

#[get("/card/sales/partials/{lang}/{name}")]
async fn get_card_sales_by_name_partials(params: web::Path<(String, String)>) -> impl Responder {
    let (lang, name) = params.into_inner();
    let mut card_ids = {
        let trie = NAME_PARTIALS_TRIE.lock().unwrap();
        trie.collect(name.to_lowercase())
    };
    card_ids.sort();
    card_ids.dedup();
    let filtered_cards_ids: Vec<String> = {
        let all_cards = ALL_CARDS.lock().unwrap();
        card_ids
            .into_iter()
            .map(|id| all_cards.get(&id).unwrap())
            .filter(|c| c.lang.eq(&lang.to_string()))
            .map(|card| card.id.clone())
            .collect()
    };
    let sales: Vec<Sale> = filtered_cards_ids
        .iter()
        .filter_map(|c| get_sales_by_card(c.to_string()).ok())
        .flatten()
        .map(Sale::from)
        .collect();
    HttpResponse::Ok().json(sales)
}