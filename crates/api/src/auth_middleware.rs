use axum::{
    extract::TypedHeader,
    http::StatusCode,
    headers::Cookie,
    http::Request,
    middleware::{Next},
    response::Response,
};

use jsonwebtoken::{decode_header, jwk::JwkSet};
pub(crate) async fn auth<B>(
    TypedHeader(cookies): TypedHeader<Cookie>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    if token_is_valid(cookies.get("hanko").unwrap_or_default()).await {
        let response = next.run(request).await;
        Ok(response)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn token_is_valid(token: &str) -> bool {
    let jwks_url_host = format!("{}/.well-known/jwks.json", std::env::var("NEXT_PUBLIC_HANKO_API_URL").unwrap_or_default());

    let jwks: JwkSet = match reqwest::get(&jwks_url_host).await {
        Ok(response) => match response.json().await {
            Ok(json) => json,
            Err(_) => return false,
        },
        Err(_) => return false,
    };

    let header = match decode_header(&token) {
        Ok(header) => header,
        Err(_) => return false,
    };

    match jwks.find(&header.kid.unwrap_or_default()) {
        Some(_) => true,
        None => return false,
    }
}