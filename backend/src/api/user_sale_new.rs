use actix_web::{HttpResponse, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;

use crate::{services::{auth_jwt::decode_jwt, database::{sale_operations::insert_sale, subscription_operations::get_subscribtions_by_card}, subscription::notify_subscription}, models::{sale::Sale, subscription::Subscription}, database::models::SqliteSale};

#[derive(Deserialize)]
pub struct SalePostData {
    pub sale_type: String,
    pub sale_object_id: String,
    pub description: String,
    pub price: i32,
    pub amount: i32,
    pub contact_type: String,
    pub location: String,
    pub web_address: String,
}

#[post("/user/sale/")]
pub async fn user_sale_new(auth: BearerAuth, body: web::Json<SalePostData>) -> HttpResponse {
    let user_id = match decode_jwt(auth.token().to_string()) {
        Some(uid) => uid,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let sqlite_sale = SqliteSale::from(body.into_inner(), user_id.clone());
    let sale = match insert_sale(sqlite_sale) {
        Ok(data) => data,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string())
    };
    // notify card subscribers of new sale
    if sale.sale_type == "CARD" {
        let sqlite_subs = match get_subscribtions_by_card(sale.sale_object_id.clone()) {
            Ok(subs) => subs,
            Err(_) => vec![],
        };
        for s in sqlite_subs.into_iter() {
            let sub = Subscription::from(s);
            match notify_subscription(sub).await {
                Ok(_) => println!("[SYSTEM] Notified user ({}) about new sale", user_id),
                Err(e) => println!("[SYSTEM] Error: Failed to notify user ({}) about new sale: {}", user_id, e.to_string()),
            };
        };
        
    }
    HttpResponse::Ok().json(Sale::from(sale))
}