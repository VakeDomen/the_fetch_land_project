use actix_web::{get, web, HttpResponse};

use crate::services::jwt::{encode_jwt, Claims, decode_jwt};




// #[get("/encode/{key}")]
// pub async fn encode_route(name: web::Path<String>) -> HttpResponse {
//     match encode_jwt() {
//         Ok(t) =>  HttpResponse::Ok().json(t),
//         Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
//     }
// }


#[get("/decode/{key}")]
pub async fn decode_route(key: web::Path<String>) -> HttpResponse {
    match decode_jwt( key.to_string() ) {
        Ok(t) =>  HttpResponse::Ok().json(t.claims),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}