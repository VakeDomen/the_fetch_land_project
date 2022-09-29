use actix_web::{HttpResponse, get, web};

use crate::{models::sale::Sale, services::database::sale_operations::{get_sales_paged_by_price, get_sales_paged_by_created}};

#[get("/card/sales/paged/{page_size}/{page_offset}/{column_sort}")]
pub async fn card_sales_paged(path_data: web::Path<(String, String, String)>) -> HttpResponse {
    let (page_size_string, page_offset_string, column_sort_name) = path_data.into_inner();
    let page_size: i64 = match page_size_string.parse::<i64>() {
        Ok(i) => i,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    let page_offset: i64 = match page_offset_string.parse::<i64>() {
        Ok(i) => i,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    let query_results = match column_sort_name.as_str() {
        "price" => get_sales_paged_by_price(page_size, page_offset),
        "created" => get_sales_paged_by_created(page_size, page_offset),
        _ => get_sales_paged_by_created(page_size, page_offset),
    };
    let sales: Vec<Sale> = match query_results {
        Ok(sales) => sales.into_iter().map(Sale::from).collect(),
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string())
    };
    HttpResponse::Ok().json(sales)
}
