use std::{error::Error, sync::Arc};

use axum::async_trait;
use dynamo_persistence_core::ClientFactory;
use service_core::{auth::UserProvider, service::Service};

use crate::persistence::{
    PersistenceLayer, PersistenceLayerFactory, UserCredRepository, UserRepository, UserTxRepository,
};

use super::{
    user_cred_repository::DynamoUserCredRepository, user_repository::DynamoUserRepository,
    user_tx_repository::DynamoUserTxRepository,
};

#[derive(Default)]
pub struct DynamoPersistenceLayerFactory {}

#[async_trait]
impl PersistenceLayerFactory for DynamoPersistenceLayerFactory {
    async fn create<S: Service>(
        &self,
        service_config: &S::ServiceConfig,
    ) -> Result<Arc<dyn PersistenceLayer>, Box<dyn Error>> {
        let sdk_config = aws_config_core::load_aws_config(service_config).await;
        let client_factory = dynamo_persistence_core::ClientFactory::new(&sdk_config);

        Ok(Arc::new(DynamoPersistenceLayer::new(&client_factory)))
    }
}

pub struct DynamoPersistenceLayer {
    user_repository: Arc<DynamoUserRepository>,
    user_cred_repository: Arc<DynamoUserCredRepository>,
    user_tx_repository: Arc<DynamoUserTxRepository>,
}

impl DynamoPersistenceLayer {
    #[allow(dead_code)]
    pub fn new(client_factory: &ClientFactory) -> Self {
        Self {
            user_repository: Arc::new(DynamoUserRepository::new(client_factory)),
            user_cred_repository: Arc::new(DynamoUserCredRepository::new(client_factory)),
            user_tx_repository: Arc::new(DynamoUserTxRepository::new(client_factory)),
        }
    }
}

impl PersistenceLayer for DynamoPersistenceLayer {
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
