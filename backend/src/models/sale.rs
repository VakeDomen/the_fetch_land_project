use serde::Serialize;

use crate::database::models::SqliteSale;


#[derive(Serialize)]
pub struct Sale {
    pub id: String,
    pub sale_type: String,
    pub user_id: String,
    pub sale_object_id: String,
    pub created: String,
    pub description: String,
    pub price: i32,
    pub amount: i32,
    pub contact_type: String,
    pub location: String,
    pub web_address: String,
}

impl Sale {
    pub fn from(sqlite_sale: SqliteSale) -> Self {
        Self { 
            id: sqlite_sale.id, 
            sale_type: sqlite_sale.sale_type, 
            user_id: sqlite_sale.user_id, 
            sale_object_id: sqlite_sale.sale_object_id, 
            location: sqlite_sale.location, 
            created: sqlite_sale.created, 
            description: sqlite_sale.description, 
            price: sqlite_sale.price, 
            amount: sqlite_sale.amount,
            contact_type: sqlite_sale.contact_type,
            web_address: sqlite_sale.web_address,            
        }
    }
}