use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;

use crate::models::{card::Card, trie::TrieTree};

use super::scryfall::{get_bulk_data, download_card_data};

// const ALL_CARDS_HASH_MAP_FILE_PATH: &str = "./small_hm.json";
const ALL_CARDS_HASH_MAP_FILE_PATH: &str = "./all_cards_hash_map.json";

pub static ALL_CARDS: Lazy<Mutex<HashMap<String, Card>>> = Lazy::new(|| {
    match serde_any::from_file(ALL_CARDS_HASH_MAP_FILE_PATH) {
        Ok(hm) => Mutex::new(hm),
        Err(_) => Mutex::new(HashMap::new())
    }
});

pub static NAME_TRIE: Lazy<Mutex<TrieTree>> = Lazy::new(|| {
    let all_cards = ALL_CARDS.lock().unwrap();
    let mut trie = TrieTree::new();
    for card in all_cards.values() {
        trie.insert(card.name.clone().to_lowercase(), card.id.clone());
    }
    Mutex::new(trie)
});

pub async fn setup_card_cache() {
    download_card_data(&get_bulk_data().await.unwrap()).await
}

pub fn save_cards_to_cache(content: &str) {
    let mut all_cards = ALL_CARDS.lock().unwrap();
    if all_cards.is_empty() {
        let cards: Vec<Card> = match serde_json::from_str(content) {
            Ok(cards) => cards,
            Err(e) => {
                println!("{}", e);
                return;
            },
        };
        println!("Parsed cards from string!");
        all_cards.clear();
        for card in cards.into_iter() {
            all_cards.insert(card.id.clone(), card);
        }
    }
}

pub fn save_cache() {
    let all_cards = ALL_CARDS.lock().unwrap();
    match serde_any::to_file(ALL_CARDS_HASH_MAP_FILE_PATH, &*all_cards) {
        Ok(_) => {},
        Err(e) => {println!("Error saving task queue: {:?}", e);}
    };
}
