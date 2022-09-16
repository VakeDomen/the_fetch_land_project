use actix_web::{HttpResponse, delete, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{services::{auth_jwt::decode_jwt, database::sale_operations::delete_sale}};

#[delete("/user/sale/{sale_id}")]
pub async fn user_sale_delete(auth: BearerAuth, sale_id: web::Path<String>) -> HttpResponse {
    let user_id = match decode_jwt(auth.token().to_string()) {
        Some(uid) => uid,
        None => return HttpResponse::Unauthorized().finish(),
    };
    
    match delete_sale(user_id, sale_id.to_string()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string())
    }
}