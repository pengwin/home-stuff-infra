use axum::async_trait;

use dynamo_persistence_core::{ClientFactory, PersistenceError, Repository};

use crate::persistence::{UserCredDbModel, UserCredRepository};

pub struct DynamoUserCredRepository {
    repository: Repository<UserCredDbModel>,
}

impl DynamoUserCredRepository {
    #[allow(dead_code)]
    pub fn new(client_factory: &ClientFactory) -> Self {
        Self {
            repository: Repository::new(client_factory),
        }
    }
}

#[async_trait]
impl UserCredRepository for DynamoUserCredRepository {
    async fn get_cred(&self, email: &str) -> Result<Option<UserCredDbModel>, PersistenceError> {
        self.repository.get(email).await
    }

    async fn get_cred_by_user_id(
        &self,
        user_id: &uuid::Uuid,
    ) -> Result<Vec<UserCredDbModel>, PersistenceError> {
        self.repository
            .query_by_index("UserIdIndex", "user_id", user_id)
            .await
    }
}
