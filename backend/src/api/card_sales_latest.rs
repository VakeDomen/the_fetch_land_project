use actix_web::{HttpResponse, get, web};

use crate::{models::sale::Sale, services::database::sale_operations::get_latest_sales};

#[get("/card/sales/latest/{num_of_sales}")]
pub async fn card_sales_latest(num_of_sales: web::Path<String>) -> HttpResponse {
    let num: i64 = match num_of_sales.parse::<i64>() {
        Ok(i) => i,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };
    let sqlite_sales = match get_latest_sales(num) {
        Ok(sales) => sales,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string())
    };
    let sales: Vec<Sale> = sqlite_sales.into_iter().map(Sale::from).collect();
    HttpResponse::Ok().json(sales)
}

#[get("/card/sales/latest/")]
pub async fn card_sales_latest_default() -> HttpResponse {
    let sqlite_sales = match get_latest_sales(10) {
        Ok(sales) => sales,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string())
    };
    let sales: Vec<Sale> = sqlite_sales.into_iter().map(Sale::from).collect();
    HttpResponse::Ok().json(sales)
}