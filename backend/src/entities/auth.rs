use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet},
    Algorithm, DecodingKey, Validation,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: Vec<String>,
    pub iat: u64,
    pub exp: u64,
    pub azp: String,
    pub scope: String,
}

#[derive(Debug)]
pub enum AuthState {
    Authorized(Claims),
    UnAuthorized,
}

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("jwt error")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("token doesn't have a `kid` header field")]
    NoKid,

    #[error("no matching jwk found for the given kid")]
    NoJwk,
}

#[derive(Debug, Clone)]
pub struct Validator {
    jwks: JwkSet,
    audience: String,
}

impl Validator {
    pub async fn new(issuer: Url, audience: String) -> reqwest::Result<Self> {
        let jwks = Self::fetch_jwks(issuer).await?;
        Ok(Self { jwks, audience })
    }

    pub async fn fetch_jwks(mut issuer: Url) -> reqwest::Result<JwkSet> {
        issuer.set_path(".well-known/jwks.json");
        let jwks = reqwest::get(issuer).await?.json().await?;
        Ok(jwks)
    }

    pub fn validate(&self, token: &str) -> Result<Claims, JwtError> {
        let header = decode_header(token)?;
        let kid = header.kid.ok_or(JwtError::NoKid)?;
        let jwk = self.jwks.find(&kid).ok_or(JwtError::NoJwk)?;

        if let AlgorithmParameters::RSA(rsa) = &jwk.algorithm {
            let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)?;

            let mut validation = Validation::new(Algorithm::from_str(
                jwk.common.key_algorithm.unwrap().to_string().as_str(),
            )?);
            validation.set_audience(&[&self.audience]);

            let token = decode::<Claims>(token, &decoding_key, &validation)?;
            Ok(token.claims)
        } else {
            unreachable!("this should be a RSA")
        }
    }
}
