use serde::Serialize;

use crate::database::models::SqliteUser;

#[derive(Serialize)]
pub struct User {
    id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
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

#[derive(Debug, Serialize)]
pub struct UserCredentials {
    pub name: String,
    pub email: String,
    pub phone: String,
}

impl UserCredentials {
    pub fn from(user: User) -> Self {
        Self { name: user.name, email: user.email, phone: user.phone }
    }
}