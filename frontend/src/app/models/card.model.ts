export interface Card {
    id: string,
    name: string,
    printed_name: string | null,
    lang: string,
    released_at: string,
    mana_cost: string | null,
    cmc: number | null,
    oracle_text: string | null,
    printed_text: string | null,
    loyalty: string | null,
    colors: string[] | null,
    color_identity: string[],
    keywords: string[],
    legalities: CardLegalities,
    reprint: boolean,
    set_id: string,
    set: string,
    set_name: string,
    collector_number: string,
    rarity: string,
    prices: CardPrices,
    highres_image: boolean,
    image_status: string,
    image_uris: CardImageUris | null,
    card_faces: CardFaces[] | null,
 }

 export interface CardImageUris {
    small: string,
    normal: string,
    large: string,
    png: string,
    art_crop: string,
    border_crop: string,
 }

export interface CardLegalities {
    standard: string,
    future: string,
    historic: string,
    gladiator: string,
    pioneer: string,
    explorer: string,
    modern: string,
    legacy: string,
    pauper: string,
    vintage: string,
    penny: string,
    commander: string,
    brawl: string,
    historicbrawl: string,
    alchemy: string,
    paupercommander: string,
    duel: string,
    oldschool: string,
    premodern: string,
}

export interface CardPrices {
    usd: string[] | null,
    usd_foil: string[] | null,
    usd_etched: string[] | null,
    eur: string[] | null,
    eur_foil: string[] | null,
    tix: string[] | null,
}

export interface CardFaces {
    artist: string | null,
    cmc: number | null,
    color_indicator: string | null,
    colors: string | null,
    flavor_text: string | null,
    illustration_id: string | null,
    image_uris: CardImageUris | null,
    layout: string | null,
    loyalty: string | null,
    mana_cost: string | null,
    name: string,
    object: string,
    oracle_id: string | null,
    oracle_text: string | null,
    power: string | null,
    printed_name: string | null,
    printed_text: string | null,
    printed_type_line: string | null,
    toughness: string | null,
    type_line: string | null,
    watermark: string | null,
 }
 