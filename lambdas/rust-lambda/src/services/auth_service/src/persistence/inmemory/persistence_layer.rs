use axum::async_trait;
use dashmap::DashMap;
use std::{error::Error, sync::Arc};

use service_core::{
    auth::{AuthConfig, UserProvider},
    service::Service,
};

use crate::persistence::{
    PersistenceLayer, PersistenceLayerFactory, UserCredRepository, UserRepository, UserTxRepository,
};

use super::{
    user_cred_repository::InmemoryUserCredRepository, user_repository::InmemoryUserRepository,
    user_tx_repository::InmemoryUserTxRepository,
};

pub struct InmemoryPersistenceLayerFactory {
    test_login: String,
    test_password: String,
    pepper: String,
}

impl InmemoryPersistenceLayerFactory {
    pub fn new<C: AuthConfig>(c: &C, test_login: &str, test_password: &str) -> Self {
        Self {
            test_login: test_login.to_owned(),
            test_password: test_password.to_owned(),
            pepper: c.pepper().to_owned(),
        }
    }
}

#[async_trait]
impl PersistenceLayerFactory for InmemoryPersistenceLayerFactory {
    async fn create<S: Service>(
        &self,
        _service_config: &S::ServiceConfig,
    ) -> Result<Arc<dyn PersistenceLayer>, Box<dyn Error>> {
        let layer = InmemoryPersistenceLayer::default();
        super::create_test_admin::create(
            &self.test_login,
            &self.test_password,
            &self.pepper,
            &layer,
        )
        .await?;
        Ok(Arc::new(layer))
    }
}

pub struct InmemoryPersistenceLayer {
    user_repository: Arc<InmemoryUserRepository>,
    user_cred_repository: Arc<InmemoryUserCredRepository>,
    user_tx_repository: Arc<InmemoryUserTxRepository>,
}

impl Default for InmemoryPersistenceLayer {
    #[allow(dead_code)]
    fn default() -> Self {
        let user_map = Arc::new(DashMap::new());
        let cred_map = Arc::new(DashMap::new());
        Self {
            user_repository: Arc::new(InmemoryUserRepository::new(user_map.clone())),
            user_cred_repository: Arc::new(InmemoryUserCredRepository::new(cred_map.clone())),
            user_tx_repository: Arc::new(InmemoryUserTxRepository::new(user_map, cred_map)),
        }
    }
}

impl PersistenceLayer for InmemoryPersistenceLayer {
    fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repository.clone()
    }

    fn user_cred_repository(&self) -> Arc<dyn UserCredRepository> {
        self.user_cred_repository.clone()
    }

    fn user_tx_repository(&self) -> Arc<dyn UserTxRepository> {
        self.user_tx_repository.clone()
    }

    fn user_provider(&self) -> Arc<dyn UserProvider> {
        self.user_repository.clone()
    }
}
