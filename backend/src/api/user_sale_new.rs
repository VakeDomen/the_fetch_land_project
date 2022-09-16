use actix_web::{HttpResponse, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;

use crate::{services::{auth_jwt::decode_jwt, database::sale_operations::insert_sale}, models::sale::Sale, database::models::SqliteSale};

#[derive(Deserialize)]
pub struct SalePostData {
    pub sale_type: String,
    pub sale_object_id: String,
    pub description: String,
    pub price: i32,
    pub amount: i32,
    pub contact_type: String,
    pub location: String,
    pub web_address: String,
}

#[post("/user/sale/")]
pub async fn user_sale_new(auth: BearerAuth, body: web::Json<SalePostData>) -> HttpResponse {
    let user_id = match decode_jwt(auth.token().to_string()) {
        Some(uid) => uid,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let sqlite_sale = SqliteSale::from(body.into_inner(), user_id);
    match insert_sale(sqlite_sale) {
        Ok(data) => HttpResponse::Ok().json(Sale::from(data)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}