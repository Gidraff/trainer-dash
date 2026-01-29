use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use std::sync::Arc;

use super::{claims::Claims, jwks::Jwks};

#[derive(Clone)]
pub struct AuthState {
    pub jwks: Arc<Jwks>,
    pub issuer: String,
    pub audience: String,
}

pub async fn auth_middleware(
    State(state): State<AuthState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    println!("DEBUG: Middleware triggered for {}", req.uri());
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let header = jsonwebtoken::decode_header(token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let kid = header.kid.ok_or(StatusCode::UNAUTHORIZED)?;

    let jwk = state
        .jwks
        .as_ref()
        .find_by_kid(&kid)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let decoding_key =
        DecodingKey::from_rsa_components(&jwk.n, &jwk.e).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let mut validation = Validation::new(header.alg);
    validation.set_issuer(&[state.issuer.as_str()]);
    validation.set_audience(&[state.audience.as_str()]);

    let token_data = match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(data) => data,
        Err(e) => {
            // THIS IS THE KEY: Check your terminal output!
            eprintln!("JWT Validation Error: {:?}", e);

            // If it says "ExpiredSignature", you just need a new token.
            // If it says "InvalidAudience", your Keycloak mapper needs adjustment.
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // 👇 Claims must be Clone
    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}
