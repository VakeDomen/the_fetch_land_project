use uuid::Uuid;

use crate::models::auth::AuthUserData;

use super::schema::users;

#[derive(Queryable, Debug)]
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