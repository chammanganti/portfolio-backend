use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
};

use crate::{
    config::AppConfig,
    errors::AuthError,
    jwt::{JwtClaims, JwtService},
};

#[derive(Debug)]
pub struct BearerAuth<'a> {
    pub claims: JwtClaims<'a>,
}

impl<'a> BearerAuth<'a> {
    pub fn new<T: Into<String>>(
        auth_header: T,
        jwt_service: &'a JwtService,
    ) -> Result<Self, AuthError> {
        Self::unwrap_bearer_auth(auth_header.into(), jwt_service)
    }

    fn unwrap_bearer_auth(
        auth_header: String,
        jwt_service: &'a JwtService,
    ) -> Result<Self, AuthError> {
        if auth_header.len() < 8 || &auth_header[..7].to_lowercase() != "bearer " {
            return Err(AuthError::InvalidHeader);
        }

        let token_data = jwt_service
            .parse_token(&auth_header[7..])
            .map_err(|_| AuthError::InvalidToken)?;

        // TODO: add checks

        Ok(Self {
            claims: token_data.claims,
        })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BearerAuth<'r> {
    type Error = AuthError;

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let app_config = request.rocket().state::<AppConfig>().unwrap();
        let auth_header = request.headers().get_one("Authorization");
        match auth_header {
            Some(auth_header) => match Self::new(auth_header, &app_config.jwt_service) {
                Ok(bearer_auth) => Outcome::Success(bearer_auth),
                Err(e) => Outcome::Failure((Status::Unauthorized, e)),
            },
            None => Outcome::Failure((Status::Unauthorized, AuthError::MissingAuthHeader)),
        }
    }
}
