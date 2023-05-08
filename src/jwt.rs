use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JwtClaims<'a> {
    pub iss: Cow<'a, str>,
    pub sub: Uuid,
    pub aud: Cow<'a, str>,
    pub exp: i64,
    pub iat: i64,
}

pub struct JwtService(String);

impl JwtService {
    pub fn new(secret_key: String) -> Self {
        Self(secret_key)
    }

    pub fn parse_token(&self, token: &str) -> Result<TokenData<JwtClaims<'_>>, ()> {
        let claims = decode::<JwtClaims<'_>>(
            token,
            &DecodingKey::from_secret(self.0.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| ())?;

        Ok(claims)
    }
}
