use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub object: String,
    pub id: String,
    pub oracle_id: Option<String>,
    pub multiverse_ids: Option<Vec<i64>>,
    pub name: String,
    pub printed_name: Option<String>,
    pub lang: String,
    pub released_at: String,
    pub uri: String,
    pub scryfall_uri: String,
    pub layout: Option<String>,
    pub highres_image: bool,
    pub image_status: String,
    pub image_uris: Option<CardImageUris>,
    pub mana_cost: Option<String>,
    pub cmc: Option<f32>,
    pub type_line: Option<String>,
    pub printed_type_line: Option<String>,
    pub oracle_text: Option<String>,
    pub printed_text: Option<String>,
    pub loyalty: Option<String>,
    pub colors: Option<Vec<String>>,
    pub color_identity: Vec<String>,
    pub keywords: Vec<String>,
    pub all_parts: Option<Vec<CardRelatedObjects>>,
    pub legalities: CardLegalities,
    pub games: Vec<String>,
    pub reserved: bool,
    pub foil: Option<bool>,
    pub nonfoil: Option<bool>,
    pub finishes: Vec<String>,
    pub oversized: bool,
    pub promo: bool,
    pub reprint: bool,
    pub variation: bool,
    pub set_id: String,
    pub set: String,
    pub set_name: String,
    pub set_type: String,
    pub set_uri: String,
    pub set_search_uri: String,
    pub scryfall_set_uri: String,
    pub rulings_uri: String,
    pub prints_search_uri: String,
    pub collector_number: String,
    pub digital: bool,
    pub rarity: String,
    pub card_back_id: Option<String>,
    pub artist: Option<String>,
    pub artist_ids: Option<Vec<String>>,
    pub illustration_id: Option<String>,
    pub border_color: String,
    pub frame: String,
    pub security_stamp: Option<String>,
    pub full_art: bool,
    pub textless: bool,
    pub booster: bool,
    pub story_spotlight: bool,
    pub promo_types: Option<Vec<String>>,
    pub edhrec_rank: Option<i64>,
    pub prices: CardPrices,
    pub related_uris: CardRelatedUris
 }


 #[derive(Debug, Serialize, Deserialize)]
 pub struct CardImageUris {
    pub small: String,
    pub normal: String,
    pub large: String,
    pub png: String,
    pub art_crop: String,
    pub border_crop: String,
 }


 #[derive(Debug, Serialize, Deserialize)]
 pub struct CardRelatedObjects {
    pub object: String,
    pub id: String,
    pub component: String,
    pub name: String,
    pub type_line: String,
    pub uri: String,
 }


 #[derive(Debug, Serialize, Deserialize)]
 pub struct CardPrices {
    pub usd: Option<String>,
    pub usd_foil: Option<String>,
    pub usd_etched: Option<String>,
    pub eur: Option<String>,
    pub eur_foil: Option<String>,
    pub tix: Option<String>,
 }


 #[derive(Debug, Serialize, Deserialize)]
 pub struct CardRelatedUris {
    pub gatherer: Option<String>,
    pub tcgplayer_infinite_articles: Option<String>,
    pub tcgplayer_infinite_decks: Option<String>,
    pub edhrec: Option<String>,
 }


 #[derive(Debug, Serialize, Deserialize)]
 pub struct CardLegalities {
    pub standard: String,
    pub future: String,
    pub historic: String,
    pub gladiator: String,
    pub pioneer: String,
    pub explorer: String,
    pub modern: String,
    pub legacy: String,
    pub pauper: String,
    pub vintage: String,
    pub penny: String,
    pub commander: String,
    pub brawl: String,
    pub historicbrawl: String,
    pub alchemy: String,
    pub paupercommander: String,
    pub duel: String,
    pub oldschool: String,
    pub premodern: String,
}