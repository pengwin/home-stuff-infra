use std::{error::Error, sync::Arc};

use axum::async_trait;
use service_core::{auth::UserProvider, service::Service};

use super::{UserCredRepository, UserRepository, UserTxRepository};

#[async_trait]
pub trait PersistenceLayerFactory: Send + Sync {
    async fn create<S: Service>(
        &self,
        service_config: &S::ServiceConfig,
    ) -> Result<Arc<dyn PersistenceLayer>, Box<dyn Error>>;
}

pub trait PersistenceLayer: Send + Sync {
    fn user_repository(&self) -> Arc<dyn UserRepository>;
    fn user_cred_repository(&self) -> Arc<dyn UserCredRepository>;
    fn user_tx_repository(&self) -> Arc<dyn UserTxRepository>;
    fn user_provider(&self) -> Arc<dyn UserProvider>;
}
