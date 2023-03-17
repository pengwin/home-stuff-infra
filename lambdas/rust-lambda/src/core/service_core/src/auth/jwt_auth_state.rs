use std::time::Duration;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use super::{auth_config::AuthConfig, claims::Claims, error::AuthError};

struct Keys {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl Keys {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
        }
    }
}

pub struct JwtAuthState {
    keys: Keys,
    expiration: Duration,
}

impl JwtAuthState {
    pub fn new<AC: AuthConfig>(config: &AC) -> Self {
        Self {
            keys: Keys::new(config.secret()),
            expiration: config.jwt_expiration(),
        }
    }

    pub fn create_token(&self, user_id: &uuid::Uuid) -> Result<String, AuthError> {
        let claims = Claims::new(user_id, &self.expiration);
        let token = encode(&Header::default(), &claims, &self.keys.encoding_key)
            .map_err(AuthError::JwtGenerateError)?;

        Ok(token)
    }

    pub fn extract_claims(&self, token: &str) -> Result<Claims, AuthError> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(token, &self.keys.decoding_key, &validation)
            .map_err(AuthError::JwtParseError)?;

        Ok(token_data.claims)
    }
}
