use actix_web::{HttpResponse, get};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{services::{auth_jwt::decode_jwt, database::sale_operations::get_sales_by_user}, models::sale::Sale};

#[get("/user/sales/")]
pub async fn user_sales(auth: BearerAuth) -> HttpResponse {
    let user_id = match decode_jwt(auth.token().to_string()) {
        Some(uid) => uid,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let sqlite_sales = match get_sales_by_user(user_id) {
        Ok(sales) => sales,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string())
    };
    let sales: Vec<Sale> = sqlite_sales.into_iter().map(Sale::from).collect();
    HttpResponse::Ok().json(sales)
}