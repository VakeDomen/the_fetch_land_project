use actix_web::{patch, HttpResponse, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};

use crate::{services::{auth_jwt::decode_jwt, database::user_operations::{update_user_info, get_user_by_id}}, models::user::User};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserPatchData {
    pub name: String,
    pub phone: String,
}

#[patch("/user/update")]
pub async fn update_user(auth: BearerAuth, body: web::Json<UserPatchData>) -> HttpResponse {
    let user_id = match decode_jwt(auth.token().to_string()) {
        Some(uid) => uid,
        None => return HttpResponse::Unauthorized().finish(),
    };
    
    match update_user_info(user_id.clone(), body.into_inner()) {
        Ok(_) => (),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let user = match get_user_by_id(user_id) {
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        Ok(user_option) => match user_option {
            Some(user) => user,
            None => return HttpResponse::InternalServerError().finish(),
        },
    };

    HttpResponse::Ok().json(User::from(user))
}