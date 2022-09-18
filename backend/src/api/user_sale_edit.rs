use actix_web::{HttpResponse, web, patch};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;

use crate::services::{auth_jwt::decode_jwt, database::sale_operations::update_sale};

#[derive(Debug, Deserialize)]
pub struct SaleEditPatchData {
    pub description: String,
    pub price: i32,
    pub amount: i32,
    pub contact_type: String,
    pub location: String,
    pub web_address: String,
}

#[patch("/user/sale/{sale_id}")]
pub async fn user_sale_edit(
    auth: BearerAuth, 
    sale_id: web::Path<String>,
    body: web::Json<SaleEditPatchData>,
) -> HttpResponse {
    let user_id = match decode_jwt(auth.token().to_string()) {
        Some(uid) => uid,
        None => return HttpResponse::Unauthorized().finish(),
    };
    
    match update_sale(sale_id.to_string(), user_id, body.0) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string())
    }
}