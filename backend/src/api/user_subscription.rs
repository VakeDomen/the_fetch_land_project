use actix_web::{HttpResponse, get};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{services::{auth_jwt::decode_jwt, database::subscription_operations::get_subscriptions_by_user}, models::subscription::Subscription};

#[get("/user/sub/")]
pub async fn user_sales(auth: BearerAuth) -> HttpResponse {
    let user_id = match decode_jwt(auth.token().to_string()) {
        Some(uid) => uid,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let sqlite_sub = match get_subscriptions_by_user(user_id) {
        Ok(subs) => subs,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string())
    };
    let subs: Vec<Subscription> = sqlite_sub.into_iter().map(Subscription::from).collect();
    HttpResponse::Ok().json(subs)
}