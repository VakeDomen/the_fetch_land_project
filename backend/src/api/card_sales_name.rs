use actix_web::{get, web, Responder, HttpResponse};

use crate::{services::{card_cache::{NAME_TRIE, ALL_CARDS}, database::sale_operaations::get_sales_by_card}, models::sale::Sale, database::models::SqliteSale};

#[get("/card/sales/name/{lang}/{name}")]
async fn get_card_sales_by_name(params: web::Path<(String, String)>) -> impl Responder {
    let (lang, name) = params.into_inner();
    let card_ids = {
        let trie = NAME_TRIE.lock().unwrap();
        trie
            .collect(name
                .to_string()
                .to_lowercase()
            )
    };
    let filtered_cards_ids: Vec<String> = {
        let all_cards = ALL_CARDS.lock().unwrap();
        card_ids
            .into_iter()
            .map(|id| all_cards.get(&id).unwrap())
            .filter(|c| c.lang.eq(&lang.to_string()))
            .map(|card| card.id.clone())
            .collect()
    };
    let sqlite_sales: Vec<SqliteSale> = filtered_cards_ids
        .iter()
        .filter_map(|c| get_sales_by_card(c.to_string()).ok())
        .flatten()
        .collect(); 
    let sales: Vec<Sale> = sqlite_sales
        .into_iter()
        .map(Sale::from)
        .collect();
    HttpResponse::Ok().json(sales)
}