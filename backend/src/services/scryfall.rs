use crate::{models::scryfall::BulkResponse, services::card_cache::{save_cards_to_cache, save_cache}};

use super::card_cache::ALL_CARDS;

pub async fn get_card_data(bulk_response: &BulkResponse) {
    // find card data url
    println!("[SCRYFALL] Parsing bulk card URL.");
    let mut card_uri = None;
    for bulk_data in bulk_response.data.iter() {
        if bulk_data.data_type.eq("all_cards") {
            card_uri = Some(bulk_data.download_uri.clone());
        }
    }
    let target = match card_uri {
        Some(u) => u,
        None => return,
    };
    // GET data
    println!("[SCRYFALL] Fetching all cards from URL.");
    let response = match reqwest::get(target).await {
        Ok(resp) => resp,
        Err(e) => {
            println!("{:#?}", e.to_string());
            return;
        }
    };
    println!("[SCRYFALL] Extracting card raw data from response...");
    match extract_card_data_string_from_resp(response).await {
        Some(c) => {
            println!("[SCRYFALL] Saving cards to cache");
            save_cards_to_cache(&c);
            println!("[SCRYFALL] Saving cards to file");
            save_cache();
        },
        None => {
            println!("Not refreshing cards");
        }
    }
    
}

async fn extract_card_data_string_from_resp(response: reqwest::Response) -> Option<String> {
    let empty = {
        let all_cards = ALL_CARDS.lock().unwrap();
        all_cards.is_empty()
    }; // ensure mutex is droppeed before await

    if empty {
        return match response.text().await {
            Ok(c) => Some(c),
            Err(e) => {
                println!("{:#?}", e.to_string());
                return None
            }
        };
    }
    None
}


pub async fn get_bulk_data() -> Option<BulkResponse> {
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