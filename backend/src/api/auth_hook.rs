use std::env;
use actix_web::{HttpResponse, get, web};
use oauth2::{
    AuthorizationCode,
    PkceCodeVerifier, TokenResponse,
};
use oauth2::reqwest::async_http_client;
use serde::Serialize;
use crate::services::database::user_operations::{insert_user, get_user_by_google_id};
use crate::models::auth::AuthUserData;
use crate::models::{auth::AuthRequest, state::AppState};
use crate::services::auth_jwt::encode_jwt;

#[derive(Serialize)]
struct JWTTokenResponse {
    token: String,
}

#[get("/auth/hook")]
pub async fn auth(
    data: web::Data<AppState>,
    params: web::Query<AuthRequest>,
) -> HttpResponse {
    let code = AuthorizationCode::new(params.code.clone());

    let pkce_code_verifier_secret = env::var("PKCE_CODE_VERIFIER").expect("Missing the PKCE_CODE_VERIFIER environment variable.");
    let pkce_code_verifier = PkceCodeVerifier::new(pkce_code_verifier_secret);
    
    let token_response = &data.oauth.exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .request_async(async_http_client)
        .await;
    
    let token = match token_response {
        Ok(resp) => resp,
        Err(e) => return HttpResponse::Ok().body(e.to_string())
    };

    let url = format!("https://www.googleapis.com/oauth2/v1/userinfo?alt=json&access_token={}", token.access_token().secret());
    let resp = match reqwest::get(url).await {
        Ok(data) => data,
        Err(e) =>  {
            println!("{:#?}", e.to_string());
            return HttpResponse::InternalServerError().finish();
        },
    };
    let text = match resp.text().await {
        Ok(s) => s,
        Err(e) =>  {
            println!("{:#?}", e.to_string());
            return HttpResponse::InternalServerError().finish();
        },
    };
    let user_data: AuthUserData = match serde_json::from_str(text.as_str()) {
        Ok(w) => w,
        Err(e) => {
            println!("{:#?}", e.to_string());
            return HttpResponse::InternalServerError().finish();
        },
    };

    let user_option = match get_user_by_google_id(user_data.id.clone()) {
        Ok(user) => user,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string())
    };
    
    let user = match user_option {
        Some(existing_user) => existing_user,
        None => match insert_user(user_data) {
            Ok(user) => user,
            Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
        }
    };
    
    match encode_jwt(user.id) {
        Ok(token) => HttpResponse::Ok().json(JWTTokenResponse { token }),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
