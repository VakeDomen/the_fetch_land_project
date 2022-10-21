use actix_web::{HttpResponse, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;

use crate::{services::{auth_jwt::decode_jwt, database::subscription_operations::insert_subscription}, models::subscription::Subscription, database::models::SqliteSubscription};

#[derive(Deserialize)]
pub struct SubscriptionPostData {
    pub sale_type: String,
    pub sale_object_id: String,
}

#[post("/user/sub/")]
pub async fn user_subscription_new(auth: BearerAuth, body: web::Json<SubscriptionPostData>) -> HttpResponse {
    let user_id = match decode_jwt(auth.token().to_string()) {
        Some(uid) => uid,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let sqlite_sub = SqliteSubscription::from(body.into_inner(), user_id);
    match insert_subscription(sqlite_sub) {
        Ok(data) => HttpResponse::Ok().json(Subscription::from(data)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}