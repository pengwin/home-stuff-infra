use std::sync::Arc;
use std::time::Duration;

use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    Extension, RequestPartsExt,
};

use jsonwebtoken::get_current_timestamp;
use serde::{Deserialize, Serialize};

use super::{error::AuthError, jwt_auth_state::JwtAuthState};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    //nbf: usize,          // Optional. Not Before (as UTC timestamp)
    sub: uuid::Uuid, // Optional. Subject (whom token refers to)
}

impl Claims {
    pub fn new(user_id: &uuid::Uuid, expiration: &Duration) -> Self {
        let current_timestamp = get_current_timestamp() as usize;
        let expiration = current_timestamp + expiration.as_secs() as usize;
        Self {
            sub: *user_id,
            aud: "test".to_owned(),
            iss: "test".to_owned(),
            exp: expiration,
            iat: current_timestamp,
        }
    }

    pub fn user_id(&self) -> &uuid::Uuid {
        &self.sub
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;

        let Extension(state) = parts.extract::<Extension<Arc<JwtAuthState>>>().await?;

        let claims = state.extract_claims(bearer.token())?;

        Ok(claims)
    }
}
