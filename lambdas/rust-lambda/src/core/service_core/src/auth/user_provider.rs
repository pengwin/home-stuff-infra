use axum::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserProviderError {
    #[error("UserProviderPersistenceError: {0:?}")]
    PersistenceError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModel {
    pub is_admin: bool,
    pub user_id: uuid::Uuid,
}

#[async_trait]
pub trait UserProvider: Send + Sync {
    async fn get_user(&self, user_id: &uuid::Uuid) -> Result<Option<UserModel>, UserProviderError>;
}
