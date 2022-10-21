use actix_web::{HttpResponse, delete, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{services::{auth_jwt::decode_jwt, database::subscription_operations::delete_subscription}};

#[delete("/user/sub/{sale_id}")]
pub async fn user_subscription_delete(auth: BearerAuth, sub_id: web::Path<String>) -> HttpResponse {
    let user_id = match decode_jwt(auth.token().to_string()) {
        Some(uid) => uid,
        None => return HttpResponse::Unauthorized().finish(),
    };
    match delete_subscription(user_id, sub_id.to_string()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string())
    }
}