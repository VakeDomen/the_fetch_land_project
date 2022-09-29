use actix_web::{HttpResponse, get};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{services::{auth_jwt::decode_jwt, database::user_operations::get_user_by_id}, models::user::User};

#[get("/user/")]
pub async fn user_get(auth: BearerAuth) -> HttpResponse {
    let user_id = match decode_jwt(auth.token().to_string()) {
        Some(uid) => uid,
        None => return HttpResponse::Unauthorized().finish(),
    };
    match get_user_by_id(user_id) {
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        Ok(user_option) => match user_option {
            Some(user) => HttpResponse::Ok().json(User::from(user)),
            None => HttpResponse::InternalServerError().finish(),
        },
    }
}