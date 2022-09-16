use actix_web::{HttpResponse, get, web};

use crate::{models::sale::Sale, services::database::sale_operations::get_sales_by_card};

#[get("/card/sales/id/{card_id}")]
pub async fn card_sales(card_id: web::Path<String>) -> HttpResponse {
    let sqlite_sales = match get_sales_by_card(card_id.into_inner()) {
        Ok(sales) => sales,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string())
    };
    let sales: Vec<Sale> = sqlite_sales.into_iter().map(Sale::from).collect();
    HttpResponse::Ok().json(sales)
}