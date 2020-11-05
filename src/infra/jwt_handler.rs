use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("{0}")]
    FetchJwks(#[from] reqwest::Error),
    #[error("{0}")]
    DecodingFailed(#[from] jsonwebtoken::errors::Error),
    #[error("kid is missing in your token")]
    KidMissing,
    #[error("JWK corresponding to the kid you provided is not found")]
    JwkNotFound,
}

#[derive(Deserialize, Debug)]
struct JsonWebKey {
    alg: Algorithm,
    kty: String,
    r#use: String,
    n: String,
    e: String,
    kid: String,
    x5t: String,
    x5c: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Jwks {
    keys: Vec<JsonWebKey>,
}

#[derive(Deserialize, Debug)]
pub struct Claims {
    pub scope: String,
    pub sub: String,
}

async fn fetch_jwks() -> Result<Jwks, JwtError> {
    let jwks_endpoint = env::var("JWKS_ENDPOINT").expect("JWKS_ENDPOINT must be set");

    let jwks = reqwest::get(&jwks_endpoint)
        .await
        .map_err(JwtError::FetchJwks)?
        .json::<Jwks>()
        .await
        .map_err(JwtError::FetchJwks)?;

    Ok(jwks)
}

pub async fn verify(token: &str) -> Result<Claims, JwtError> {
    let jwt_audience = env::var("JWT_AUDIENCE").expect("JWT_AUDIENCE must be set");
    let jwt_issuer = env::var("JWT_ISSUER").expect("JWT_ISSUER must be set");

    let header = decode_header(token)?;
    let kid = header.kid.ok_or(JwtError::KidMissing)?;

    let jwks = fetch_jwks().await?;
    let jwk = jwks
        .keys
        .iter()
        .find(|jwk| jwk.kid == kid)
        .ok_or(JwtError::JwkNotFound)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[jwt_audience]);
    let validation = Validation {
        iss: Some(jwt_issuer),
        ..validation
    };

    let token_message = decode::<Claims>(
        token,
        &DecodingKey::from_rsa_components(&jwk.n, &jwk.e),
        &validation,
    )?;

    Ok(token_message.claims)
}
