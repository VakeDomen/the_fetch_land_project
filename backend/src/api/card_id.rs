use actix_web::{get, web, Responder, HttpResponse};

use crate::{services::card_cache::ALL_CARDS, models::card::CardPublic};

#[get("/card/id/{id}")]
async fn get_card(id: web::Path<String>) -> impl Responder {
    let all_cards = ALL_CARDS.lock().unwrap();
    let card = all_cards.get(&id.into_inner());
    match card {
        Some(c) => HttpResponse::Ok().json(CardPublic::from(c)),
        None => HttpResponse::NotFound().finish()
    }
}