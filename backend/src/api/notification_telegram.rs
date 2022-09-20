use actix_web::{HttpResponse, post, web};
use serde::Deserialize;

use crate::services::telegram::notify_via_telegram;

#[derive(Debug, Deserialize)]
pub struct NotificationPostData {
    pub message: String,
    pub sender: String,
}

#[post("/notify/")]
pub async fn notify_telegram(
    body: web::Json<NotificationPostData>,
) -> HttpResponse {   
    match notify_via_telegram(body.message.clone(), body.sender.clone()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string())
    }
}