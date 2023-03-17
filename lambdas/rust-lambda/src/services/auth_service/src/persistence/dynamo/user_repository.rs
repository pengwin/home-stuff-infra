use axum::async_trait;

use dynamo_persistence_core::{ClientFactory, PersistenceError, Repository};
use service_core::auth::{UserModel, UserProvider, UserProviderError};

use crate::persistence::{UserDbModel, UserRepository};

pub struct DynamoUserRepository {
    repository: Repository<UserDbModel>,
}

impl DynamoUserRepository {
    #[allow(dead_code)]
    pub fn new(client_factory: &ClientFactory) -> Self {
        Self {
            repository: Repository::new(client_factory),
        }
    }
}

#[async_trait]
impl UserRepository for DynamoUserRepository {
    async fn get_all(&self) -> Result<Vec<UserDbModel>, PersistenceError> {
        self.repository.get_all().await
    }

    async fn get_user(
        &self,
        user_id: &uuid::Uuid,
    ) -> Result<Option<UserDbModel>, PersistenceError> {
        self.repository.get(user_id).await
    }
}

#[async_trait]
impl UserProvider for DynamoUserRepository {
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
