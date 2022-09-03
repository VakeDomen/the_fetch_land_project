#[derive(Queryable, Debug)]
pub struct Sale {
    pub id: String,
    pub sale_type: String,
    pub user_id: String,
    pub sale_object_id: String,
    pub location_coords: String,
    pub created: String,
    pub price: i32,
    pub description: String,
}

#[derive(Queryable, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

