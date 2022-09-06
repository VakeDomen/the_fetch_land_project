use serde::Serialize;

use crate::database::models::SqliteUser;

#[derive(Serialize)]
pub struct User {
    id: String,
    name: String,
    email: String,
    phone: String,
    img: String,
}

impl User {
    pub fn from(sqlite_user: SqliteUser) -> Self {
        Self {
            id: sqlite_user.id,
            name: sqlite_user.name,
            email: sqlite_user.email,
            phone: sqlite_user.phone,
            img: sqlite_user.img,
        }
    }
}