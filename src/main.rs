use std::{fs::File, path::Path, io::Write};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::models::scryfall::{BulkData, BulkResponse};
mod models;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_cards)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/c")]
async fn get_cards() -> impl Responder {
    let bulk = match get_bulk_data().await {
        Some(data) => data,
        None => return HttpResponse::InternalServerError().finish(),
    };
    let string = match serde_json::to_string(&bulk) {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().finish(),
    };
    let data_zip = download_card_data(&bulk).await;
    HttpResponse::Ok().json(bulk)

}

async fn download_card_data(bulk_response: &BulkResponse) -> Option<File> {
    let mut card_uri = None;
    for bulk_data in bulk_response.data.iter() {
        if bulk_data.data_type.eq("all_cards") {
            card_uri = Some(bulk_data.download_uri.clone());
        }
    }
    let target = match card_uri {
        Some(u) => u,
        None => return None
    };
    println!("Dowanloading cards");
    let response = match reqwest::get(target).await {
        Ok(resp) => resp,
        Err(e) => {
            println!("{:#?}", e.to_string());
            return None;
        }
    };
    println!("Creating card file");
    let path = Path::new("./all_cards.json");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let content = match response.text().await {
        Ok(c) => c,
        Err(e) => {
            println!("{:#?}", e.to_string());
            return None;
        }
    };
    println!("Writing cards to file");
    match file.write_all(content.as_bytes()) {
        Ok(res) => {
            println!("Done!");
            Some(file)
        },
        Err(e) =>  {
            println!("{:#?}", e.to_string());
            return None;
        }
    }
}

async fn get_bulk_data() -> Option<BulkResponse> {
    let resp = match reqwest::get("https://api.scryfall.com/bulk-data").await {
        Ok(data) => data,
        Err(e) =>  {
            println!("{:#?}", e.to_string());
            return None;
        },
    };
    let text = match resp.text().await {
        Ok(s) => s,
        Err(e) =>  {
            println!("{:#?}", e.to_string());
            return None;
        },
    };
    match serde_json::from_str(text.as_str()) {
        Ok(w) => Some(w),
        Err(e) => {
            println!("{:#?}", e.to_string());
            None
        },
    } 
}