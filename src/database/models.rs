use chrono::Utc;
use uuid::Uuid;

use crate::{models::auth::AuthUserData, api::user_sale_new::SalePostData};

use super::schema::users;

#[derive(Queryable, Debug, Insertable)]
pub struct SqliteSale {
    pub id: String,
    pub sale_type: String,
    pub user_id: String,
    pub sale_object_id: String,
    pub location_coords: String,
    pub created: String,
    pub description: String,
    pub price: i32,
}

impl SqliteSale {
    pub fn from(post_data: SalePostData, uid: String) -> Self {
        Self { 
            id: Uuid::new_v4().to_string(), 
            sale_type: post_data.sale_type, 
            user_id: uid, 
            sale_object_id: post_data.sale_object_id, 
            location_coords: post_data.location_coords, 
            created: Utc::now().naive_utc().to_string(), 
            description: post_data.description, 
            price: post_data.price 
        }
    }
}

#[derive(Queryable, Debug, Identifiable, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct SqliteUser {
    pub id: String,
    pub google_id: String,
    pub img: String,
    pub email: String,
    pub name: String,
    pub phone: String,
}

impl SqliteUser {
    pub fn from(google_user_data: AuthUserData) -> Self {
        SqliteUser { 
            id: Uuid::new_v4().to_string(), 
            google_id: google_user_data.id, 
            img: google_user_data.picture,
            name: "".to_string(), 
            email: google_user_data.email, 
            phone: "".to_string()
        }
    }
}