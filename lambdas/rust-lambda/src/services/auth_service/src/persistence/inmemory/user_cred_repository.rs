use std::sync::Arc;

use axum::async_trait;
use dashmap::DashMap;

use dynamo_persistence_core::PersistenceError;

use crate::persistence::{UserCredDbModel, UserCredRepository};

pub struct InmemoryUserCredRepository {
    cred_map: Arc<DashMap<String, UserCredDbModel>>,
}

impl InmemoryUserCredRepository {
    #[allow(dead_code)]
    pub fn new(cred_map: Arc<DashMap<String, UserCredDbModel>>) -> Self {
        Self { cred_map }
    }
}

#[async_trait]
impl UserCredRepository for InmemoryUserCredRepository {
    async fn get_cred(&self, email: &str) -> Result<Option<UserCredDbModel>, PersistenceError> {
        let cred = self.cred_map.get(email).map(|c| c.clone());

        Ok(cred)
    }

    async fn get_cred_by_user_id(
        &self,
        user_id: &uuid::Uuid,
    ) -> Result<Vec<UserCredDbModel>, PersistenceError> {
        let cred = self
            .cred_map
            .iter()
            .filter(|t| t.user_id.eq(user_id))
            .map(|t| t.clone())
            .collect();

        Ok(cred)
    }
}
