use std::env;
use actix_web::{HttpResponse, get, web};
use actix_web::http::header;
use oauth2::{
    CsrfToken, 
    Scope, 
    PkceCodeChallenge, 
    PkceCodeVerifier,
};
use crate::models::state::AppState;

#[get("/login")]
pub async fn login(data: web::Data<AppState>) -> HttpResponse {
    // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    // let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    let pkce_code_verifier_secret = env::var("PKCE_CODE_VERIFIER").expect("Missing the PKCE_CODE_VERIFIER environment variable.");
    let pkce_code_verifier = PkceCodeVerifier::new(pkce_code_verifier_secret);
    let pkce_code_challenge = PkceCodeChallenge::from_code_verifier_sha256(&pkce_code_verifier);
    // session.insert("pkce_verifier", pkce_code_verifier);
    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, _csrf_state) = &data
        .oauth
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the "calendar" features and the user's profile.
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))        
        .set_pkce_challenge(pkce_code_challenge)
        .url();
    HttpResponse::Found()
        .append_header((header::LOCATION, authorize_url.to_string()))
        .finish()
}