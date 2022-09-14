import { Card } from "./card.model";

export interface CardSale {
    id: string,
    sale_type: string,
    user_id: string,
    sale_object_id: string,
    location_coords: string,
    created: string,
    description: string,
    price: number,
    amount: number,
    card: Card,
}