use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct CardPublic {
   pub id: String,
   pub name: String,
   pub printed_name: Option<String>,
   pub lang: String,
   pub released_at: String,
   pub highres_image: Option<bool>,
   pub image_status: Option<String>,
   pub image_uris: Option<CardImageUris>,
   pub mana_cost: Option<String>,
   pub cmc: Option<f32>,
   pub oracle_text: Option<String>,
   pub printed_text: Option<String>,
   pub loyalty: Option<String>,
   pub colors: Option<Vec<String>>,
   pub color_identity: Vec<String>,
   pub keywords: Vec<String>,
   pub legalities: CardLegalities,
   pub reprint: bool,
   pub set_id: String,
   pub set: String,
   pub set_name: String,
   pub collector_number: String,
   pub rarity: String,
   pub prices: CardPrices,
   pub card_faces: Option<Vec<CardFace>>
}


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
   pub highres_image: Option<bool>,
   pub image_status: Option<String>,
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
   pub related_uris: CardRelatedUris,
   pub card_faces: Option<Vec<CardFace>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardFace {
   pub image_uris: Option<CardImageUris>,
   pub layout: Option<String>,
   pub name: Option<String>,
   pub object: Option<String>,
   pub oracle_id: Option<String>,
   pub printed_name: Option<String>,
}

 #[derive(Debug, Serialize, Deserialize)]
 pub struct CardImageUris {
    pub small: Option<String>,
    pub normal: Option<String>,
    pub large: Option<String>,
    pub png: Option<String>,
    pub art_crop: Option<String>,
    pub border_crop: Option<String>,
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

impl CardPublic {
   pub fn from(card: &Card) -> Self {
      CardPublic { 
         id: card.id.clone(), 
         name: card.name.clone(), 
         printed_name: card.printed_name.clone(), 
         lang: card.lang.clone(), 
         released_at: card.released_at.clone(), 
         mana_cost: card.mana_cost.clone(), 
         cmc: card.cmc, 
         oracle_text: card.oracle_text.clone(), 
         printed_text: card.printed_text.clone(), 
         loyalty: card.loyalty.clone(), 
         colors: card.colors.clone(), 
         color_identity: card.color_identity.clone(), 
         keywords: card.keywords.clone(), 
         legalities: CardLegalities { 
            standard: card.legalities.standard.clone(), 
            future: card.legalities.future.clone(), 
            historic: card.legalities.historic.clone(), 
            gladiator: card.legalities.gladiator.clone(), 
            pioneer: card.legalities.pioneer.clone(), 
            explorer: card.legalities.explorer.clone(), 
            modern: card.legalities.modern.clone(), 
            legacy: card.legalities.legacy.clone(), 
            pauper: card.legalities.pauper.clone(), 
            vintage: card.legalities.vintage.clone(), 
            penny: card.legalities.penny.clone(), 
            commander: card.legalities.commander.clone(), 
            brawl: card.legalities.brawl.clone(), 
            historicbrawl: card.legalities.historicbrawl.clone(), 
            alchemy: card.legalities.alchemy.clone(), 
            paupercommander: card.legalities.paupercommander.clone(), 
            duel: card.legalities.duel.clone(), 
            oldschool: card.legalities.oldschool.clone(), 
            premodern: card.legalities.premodern.clone(),
         },
         reprint: card.reprint, 
         set_id: card.set_id.clone(), 
         set: card.set.clone(), 
         set_name: card.set_name.clone(), 
         collector_number: card.collector_number.clone(), 
         rarity: card.rarity.clone(), 
         prices: CardPrices { 
            usd: card.prices.usd.clone(), 
            usd_foil: card.prices.usd_foil.clone(), 
            usd_etched: card.prices.usd_etched.clone(), 
            eur: card.prices.eur.clone(), 
            eur_foil: card.prices.eur_foil.clone(), 
            tix: card.prices.tix.clone() 
         },
        highres_image: card.highres_image.clone(),
        image_status: card.image_status.clone(),
        image_uris: if let Some(uris) = &card.image_uris { Some(CardImageUris {
            small: uris.small.clone(),
            normal: uris.normal.clone(),
            large: uris.large.clone(),
            png: uris.png.clone(),
            art_crop: uris.art_crop.clone(),
            border_crop: uris.border_crop.clone(),
        })} else { None },
        card_faces:
        if let Some(faces) = &card.card_faces { 
         Some(faces.iter().map(|face| CardFace {
            image_uris: if let Some(uris) = &face.image_uris { Some(CardImageUris {
               small: uris.small.clone(),
               normal: uris.normal.clone(),
               large: uris.large.clone(),
               png: uris.png.clone(),
               art_crop: uris.art_crop.clone(),
               border_crop: uris.border_crop.clone(),
           })} else { None },
            layout: face.layout.clone(),
            name: face.name.clone(),
            object: face.object.clone(),
            oracle_id: face.oracle_id.clone(),
            printed_name: face.printed_name.clone(),
        }).collect() )
      } else { None }
      }
   }
}
