use axum::async_trait;

use dynamo_persistence_core::PersistenceError;

use super::{UserCredDbModel, UserDbModel};

#[async_trait]
pub trait UserTxRepository: Send + Sync {
    async fn save(
        &self,
        user: &UserDbModel,
        user_cred: &UserCredDbModel,
    ) -> Result<(), PersistenceError>;
    async fn delete(&self, user_uuid: &uuid::Uuid, email: &str) -> Result<(), PersistenceError>;
}
