mod admin_claims;
mod auth_config;
mod claims;
mod error;
mod jwt_auth_state;
mod user_provider;

use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

pub use self::admin_claims::AdminClaims;
pub use self::auth_config::AuthConfig;
pub use self::claims::Claims;
pub use self::error::AuthError;
pub use self::jwt_auth_state::JwtAuthState;
pub use self::user_provider::{UserModel, UserProvider, UserProviderError};

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct AuthRequest {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct AuthErrorResponse {
    pub error: String,
}
