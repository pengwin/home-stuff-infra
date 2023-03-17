use axum::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use dynamo_persistence_core::{DbModel, PersistenceError};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct UserCredDbModel {
    pub email: String,
    pub user_id: uuid::Uuid,
    pub password_hash: String,
    pub salt: String,
    pub created_by: Option<uuid::Uuid>,
    pub created_utc: DateTime<Utc>,
    pub updated_utc: DateTime<Utc>,
}

impl DbModel for UserCredDbModel {
    fn table() -> String {
        "user_cred".to_owned()
    }
    fn hash_key() -> String {
        "email".to_owned()
    }
}

#[async_trait]
pub trait UserCredRepository: Send + Sync {
    async fn get_cred(&self, email: &str) -> Result<Option<UserCredDbModel>, PersistenceError>;

    async fn get_cred_by_user_id(
        &self,
        user_id: &uuid::Uuid,
    ) -> Result<Vec<UserCredDbModel>, PersistenceError>;
}
