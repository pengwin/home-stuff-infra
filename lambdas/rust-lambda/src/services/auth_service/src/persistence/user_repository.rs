use axum::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use dynamo_persistence_core::{DbModel, PersistenceError};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct UserDbModel {
    pub user_id: uuid::Uuid,
    pub username: String,
    pub is_admin: bool,
    pub created_by: Option<uuid::Uuid>,
    pub created_utc: DateTime<Utc>,
    pub updated_utc: DateTime<Utc>,
}

impl DbModel for UserDbModel {
    fn table() -> String {
        "user".to_owned()
    }
    fn hash_key() -> String {
        "user_id".to_owned()
    }
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<UserDbModel>, PersistenceError>;

    async fn get_user(&self, user_id: &uuid::Uuid)
        -> Result<Option<UserDbModel>, PersistenceError>;
}
