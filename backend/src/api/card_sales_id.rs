use actix_web::{get, web, Responder, HttpResponse};

use crate::{services::{database::sale_operations::get_sales_by_card}, models::sale::Sale};

#[get("/card/sales/id/{id}")]
async fn get_card_sales_by_id(id: web::Path<String>) -> impl Responder {
    let sqlite_sales = match get_sales_by_card(id.to_string()) {
        Ok(sales) => sales,
        Err(_) => return HttpResponse::NoContent().finish(),
    };
    let sales: Vec<Sale> = sqlite_sales
        .into_iter()
        .map(Sale::from)
        .collect();
    HttpResponse::Ok().json(sales)
}
