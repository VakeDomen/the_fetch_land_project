use std::{collections::HashMap, sync::Mutex, fs::File,time::SystemTime, io::{BufWriter, BufReader}};

use bincode::serialize_into;
use once_cell::sync::Lazy;

use crate::models::{card::Card, trie::TrieTree};

use super::scryfall::{get_bulk_data, get_card_data};

// const ALL_CARDS_HASH_MAP_FILE_PATH: &str = "./small_hm.json";
const ALL_CARDS_HASH_MAP_FILE_PATH: &str = "./all_cards_hash_map.json";

pub static ALL_CARDS: Lazy<Mutex<HashMap<String, Card>>> = Lazy::new(|| {
    let t0 = SystemTime::now();
    let file = match File::open(ALL_CARDS_HASH_MAP_FILE_PATH) {
        Ok(f) => f,
        Err(e) => {
            println!("[CACHE] Error reading HashMap file: {}", e.to_string());
            return Mutex::new(HashMap::new())
        },
    };
    match bincode::deserialize_from(BufReader::new(file)) {
        Ok(hm) => {
            match t0.elapsed() {
                Ok(elapsed) => println!("[CACHE] Took {} seconds to load HashMap from disk", elapsed.as_secs()),
                Err(e) => println!("[CACHE] Error: {e:?}"),
            }
            Mutex::new(hm)
        },
        Err(e) => {
            println!("[CACHE] Error loading HashMap from disk: {}", e.to_string());
            Mutex::new(HashMap::new())
        }
    }
});

pub static NAME_TRIE: Lazy<Mutex<TrieTree>> = Lazy::new(|| {
    let all_cards = ALL_CARDS.lock().unwrap();
    let mut trie = TrieTree::new();
    for card in all_cards.values() {
        trie.insert(card.name.clone().to_lowercase(), card.id.clone(), false);
    }
    Mutex::new(trie)
});

pub static NAME_PARTIALS_TRIE: Lazy<Mutex<TrieTree>> = Lazy::new(|| {
    let all_cards = ALL_CARDS.lock().unwrap();
    let mut trie = TrieTree::new();
    for card in all_cards.values() {
        trie.insert(card.name.clone().to_lowercase(), card.id.clone(), true);
    }
    Mutex::new(trie)
});

pub async fn setup_card_cache() {
    get_card_data(&get_bulk_data().await.unwrap()).await
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
        println!("[CACHE] Parsed cards from string!");
        all_cards.clear();
        for card in cards.into_iter() {
            all_cards.insert(card.id.clone(), card);
        }
    }
}

pub fn save_cache() {
    let t0 = SystemTime::now();
    let all_cards = ALL_CARDS.lock().unwrap();
    let mut f = BufWriter::new(File::create(ALL_CARDS_HASH_MAP_FILE_PATH).unwrap());
    match serialize_into(&mut f, &*all_cards) {
        Ok(_) => {
            match t0.elapsed() {
                Ok(elapsed) => println!("[CACHE] Took {} seconds to save  hashmap to disk", elapsed.as_secs()),
                Err(e) => println!("[CACHE] Error: {e:?}"),
            }
        },
        Err(e) => {println!("[CACHE] Error saving cards to file: {:?}", e);}
    };
}
