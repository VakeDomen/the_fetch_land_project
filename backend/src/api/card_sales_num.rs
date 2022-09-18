use actix_web::{HttpResponse, get};

use crate::{services::database::sale_operations::get_num_of_sales};

#[get("/card/sales/num/")]
pub async fn card_sales_num() -> HttpResponse {
    let num_of_sales = match get_num_of_sales() {
        Ok(sales) => sales,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string())
    };
    HttpResponse::Ok().json(num_of_sales)
}