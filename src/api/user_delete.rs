use actix_web::{HttpResponse, delete};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{services::{auth_jwt::decode_jwt, database::{user_operations::delete_user, sale_operaations::delete_all_user_sales}}};

#[delete("/user/")]
pub async fn user_delete(auth: BearerAuth) -> HttpResponse {
    let user_id = match decode_jwt(auth.token().to_string()) {
        Some(uid) => uid,
        None => return HttpResponse::Unauthorized().finish(),
    };
    
    match delete_all_user_sales(user_id.clone()) {
        Ok(_) => (),
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string())
    }

    match delete_user(user_id) {
        Ok(_) => (),
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string())
    }

    HttpResponse::Ok().finish()
}