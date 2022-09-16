import { Card } from "./card.model";

export interface CardSale {
    id: string,
    sale_type: string,
    user_id: string,
    sale_object_id: string,
    created: string,
    description: string,
    price: number,
    amount: number,
    card: Card,
    contact_type: 'WEB' | 'PHONE' | 'EMAIL',
    location: string,
    web_address: string,
}