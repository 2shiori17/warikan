// https://github.com/tokio-rs/axum/blob/main/examples/jwt/src/main.rs
// https://github.com/Keats/jsonwebtoken/blob/master/examples/auth0.rs

use crate::app;
use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet},
    Algorithm, DecodingKey, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("cannot extract `Authorization Bearer` header field")]
    Extract,

    #[error("jwt error")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("token doesn't have a `kid` header field")]
    NoKid,

    #[error("no matching jwk found for the given kid")]
    NoJwk,
}

#[derive(Debug)]
pub enum AuthState {
    Authorized(Claims),
    UnAuthorized,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    sub: String,
    aud: Vec<String>,
    iat: u64,
    exp: u64,
    azp: String,
    scope: String,
}

#[async_trait]
impl FromRequestParts<app::State> for AuthState {
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut Parts,
        state: &app::State,
    ) -> Result<Self, Self::Rejection> {
        Ok(match validate(parts, state).await {
            Ok(token) => AuthState::Authorized(token.claims),
            Err(err) => {
                dbg!(err);
                AuthState::UnAuthorized
            }
        })
    }
}

pub async fn fetch_jwks(mut issuer: Url) -> reqwest::Result<JwkSet> {
    issuer.set_path(".well-known/jwks.json");
    let jwks = reqwest::get(issuer).await?.json().await?;
    Ok(jwks)
}

async fn validate(parts: &mut Parts, state: &app::State) -> Result<TokenData<Claims>, AuthError> {
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .or(Err(AuthError::Extract))?;
    let token = bearer.token();

    let header = decode_header(token)?;
    let kid = header.kid.ok_or(AuthError::NoKid)?;
    let jwk = state.jwks.find(&kid).ok_or(AuthError::NoJwk)?;

    if let AlgorithmParameters::RSA(rsa) = &jwk.algorithm {
        let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)?;

        let mut validation = Validation::new(Algorithm::from_str(
            jwk.common.key_algorithm.unwrap().to_string().as_str(),
        )?);
        validation.set_audience(&[&state.auth0_audience]);
        // validation.validate_exp = false;

        let token = decode::<Claims>(token, &decoding_key, &validation)?;
        Ok(token)
    } else {
        unreachable!("this should be a RSA")
    }
}
