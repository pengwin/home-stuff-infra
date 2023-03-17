use std::sync::Arc;

use axum::{
    async_trait, extract::FromRequestParts, http::request::Parts, Extension, RequestPartsExt,
};

use serde::{Deserialize, Serialize};

use super::{
    claims::Claims,
    error::AuthError,
    user_provider::{UserModel, UserProvider},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminClaims {
    claims: Claims,
    user: UserModel,
}

impl AdminClaims {
    pub fn user_id(&self) -> &uuid::Uuid {
        &self.user.user_id
    }

    pub fn is_admin(&self, id: &uuid::Uuid) -> bool {
        self.user.user_id.eq(id)
    }

    async fn get_user(
        user_provider: Arc<dyn UserProvider>,
        claims: &Claims,
    ) -> Result<UserModel, AuthError> {
        let user = user_provider
            .get_user(claims.user_id())
            .await
            .map_err(|e| AuthError::PersistenceError(format!("{:?}", e)))?
            .map_or(
                Result::<UserModel, AuthError>::Err(AuthError::WrongCredentials),
                Result::<UserModel, AuthError>::Ok,
            )?;

        Ok(user)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AdminClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, state).await?;

        let Extension(user_provider) = parts.extract::<Extension<Arc<dyn UserProvider>>>().await?;

        let user = Self::get_user(user_provider, &claims).await?;
        if !user.is_admin {
            return Err(AuthError::WrongRole);
        }

        let admin_claims = AdminClaims { claims, user };
        Ok(admin_claims)
    }
}
