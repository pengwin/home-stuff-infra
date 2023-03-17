use dashmap::DashMap;
use std::sync::Arc;

use axum::async_trait;
use dynamo_persistence_core::PersistenceError;

use crate::persistence::{UserCredDbModel, UserDbModel, UserTxRepository};

pub struct InmemoryUserTxRepository {
    user_map: Arc<DashMap<uuid::Uuid, UserDbModel>>,
    cred_map: Arc<DashMap<String, UserCredDbModel>>,
}

impl InmemoryUserTxRepository {
    #[allow(dead_code)]
    pub fn new(
        user_map: Arc<DashMap<uuid::Uuid, UserDbModel>>,
        cred_map: Arc<DashMap<String, UserCredDbModel>>,
    ) -> Self {
        Self { user_map, cred_map }
    }
}

#[async_trait]
impl UserTxRepository for InmemoryUserTxRepository {
    async fn save(
        &self,
        user: &UserDbModel,
        user_cred: &UserCredDbModel,
    ) -> Result<(), PersistenceError> {
        self.user_map.insert(user.user_id, user.clone());
        self.cred_map
            .insert(user_cred.email.clone(), user_cred.clone());

        Ok(())
    }

    async fn delete(&self, user_uuid: &uuid::Uuid, email: &str) -> Result<(), PersistenceError> {
        self.user_map.remove(user_uuid);
        self.cred_map.remove(email);

        Ok(())
    }
}
