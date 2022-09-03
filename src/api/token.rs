use std::env;
use actix_web::{HttpResponse, get, web};
use actix_web::http::header;
use oauth2::{
    AuthorizationCode,
    CsrfToken, 
    Scope, 
    PkceCodeChallenge, 
    PkceCodeVerifier,
};
use oauth2::reqwest::async_http_client;
use crate::models::{auth::AuthRequest, state::AppState};

#[get("/auth")]
pub async fn auth(
    data: web::Data<AppState>,
    params: web::Query<AuthRequest>,
) -> HttpResponse {
    let code = AuthorizationCode::new(params.code.clone());
    let state = CsrfToken::new(params.state.clone());
    let _scope = params.scope.clone();
    println!("scope {:#?}", _scope);
    
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
    // println!("{:#?}", token.access_token().secret());
    // TODO: send access token to https://www.googleapis.com/oauth2/v1/userinfo?alt=json&access_token=
    // to get the user info
    let html = format!(
        r#"<html>
        <head><title>OAuth2 Test</title></head>
        <body>
            Google returned the following state:
            <pre>{}</pre>
            Google returned the following token:
            <pre>{:?}</pre>
        </body>
    </html>"#,
        state.secret(),
        token
    );
    HttpResponse::Ok().body(html)
}

#[get("/login")]
pub async fn login(data: web::Data<AppState>) -> HttpResponse {
    // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    // let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    let pkce_code_verifier_secret = env::var("PKCE_CODE_VERIFIER").expect("Missing the PKCE_CODE_VERIFIER environment variable.");
    let pkce_code_verifier = PkceCodeVerifier::new(pkce_code_verifier_secret);
    let pkce_code_challenge = PkceCodeChallenge::from_code_verifier_sha256(&pkce_code_verifier);
    // session.insert("pkce_verifier", pkce_code_verifier);
    println!("VERIFIER {:#?}", pkce_code_verifier.secret().to_string());
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
    
    println!("{}", authorize_url);
    HttpResponse::Found()
        .append_header((header::LOCATION, authorize_url.to_string()))
        .finish()
}