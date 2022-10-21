use serde::Serialize;

use crate::database::models::SqliteSubscription;

#[derive(Serialize)]
pub struct Subscription {
    pub id: String,
    pub sale_type: String,
    pub user_id: String,
    pub sale_object_id: String,
    pub created: String,
}

impl Subscription {
    pub fn from(sqlite_sub: SqliteSubscription) -> Self {
        Self { 
            id: sqlite_sub.id, 
            sale_type: sqlite_sub.sale_type, 
            user_id: sqlite_sub.user_id, 
            sale_object_id: sqlite_sub.sale_object_id, 
            created: sqlite_sub.created,      
        }
    }
}