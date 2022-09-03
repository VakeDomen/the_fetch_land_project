use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
    pub scope: String,
}

#[derive(Deserialize)]
pub struct AuthData {
    pub id:	String,
    pub email:	String,
    pub verified_email:	bool,
    pub name:	String,
    pub given_name:	String,
    pub family_name:	String,
    pub picture:	String,
    pub locale:	String,
}