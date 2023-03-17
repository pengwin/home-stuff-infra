use std::sync::Arc;

use axum::async_trait;
use dashmap::DashMap;

use dynamo_persistence_core::PersistenceError;
use service_core::auth::{UserModel, UserProvider, UserProviderError};

use crate::persistence::{UserDbModel, UserRepository};

pub struct InmemoryUserRepository {
    user_map: Arc<DashMap<uuid::Uuid, UserDbModel>>,
}

impl InmemoryUserRepository {
    #[allow(dead_code)]
    pub fn new(user_map: Arc<DashMap<uuid::Uuid, UserDbModel>>) -> Self {
        Self { user_map }
    }
}

#[async_trait]
impl UserRepository for InmemoryUserRepository {
    async fn get_all(&self) -> Result<Vec<UserDbModel>, PersistenceError> {
        let users = self.user_map.iter().map(|t| t.clone()).collect();

        Ok(users)
    }

    async fn get_user(
        &self,
        user_id: &uuid::Uuid,
    ) -> Result<Option<UserDbModel>, PersistenceError> {
        let user = self.user_map.get(user_id).map(|c| c.clone());

        Ok(user)
    }
}

#[async_trait]
impl UserProvider for InmemoryUserRepository {
    async fn get_user(&self, user_id: &uuid::Uuid) -> Result<Option<UserModel>, UserProviderError> {
        UserRepository::get_user(self, user_id)
            .await
            .map(|o| {
                o.map(|u| UserModel {
                    is_admin: u.is_admin,
                    user_id: u.user_id,
                })
            })
            .map_err(|e| UserProviderError::PersistenceError(format!("{:?}", e)))
    }
}
