use actix_web::{HttpResponse, get, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{services::{auth_jwt::decode_jwt, database::user_operations::get_user_by_id}, models::user::{UserCredentials, User}};

#[get("/user/credentials/{uid}")]
pub async fn user_credentials(auth: BearerAuth, uid: web::Path<String>) -> HttpResponse {
    match decode_jwt(auth.token().to_string()) {
        Some(_) => (),
        None => return HttpResponse::Unauthorized().finish(),
    };
    
    match get_user_by_id(uid.to_string()) {
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        Ok(user_option) => match user_option {
            Some(user) => HttpResponse::Ok().json(UserCredentials::from(User::from(user))),
            None => HttpResponse::InternalServerError().finish(),
        },
    }
}