use config_core::ServiceConfig;
use std::{
    error::Error as StdError,
    net::{AddrParseError, SocketAddr},
};
use thiserror::Error;

use axum::{async_trait, Router};

use crate::graceful_shutdown;

#[derive(Error, Debug)]
pub enum ServiceRunError {
    #[error("AddrParseError: {0:?}")]
    AddrParseError(#[from] AddrParseError),
    #[error("ServiceConfigError: {0:?}")]
    ServiceConfigError(#[from] config_core::ConfigError),
    #[error("RouterCreationError: {0:?}")]
    RouterCreationError(Box<dyn StdError + Send + Sync>),
    #[error("MissingEndpointConfiguration")]
    MissingEndpointConfiguration,
    #[error("ServeError: {0:?}")]
    ServeError(Box<dyn StdError + Send + Sync>),
}

#[async_trait]
pub trait Service: Send + Sync {
    type ServiceConfig: ServiceConfig + Send + Sync;
    type CreateRouterError: StdError + Send + Sync + 'static;

    async fn create_router(
        &self,
        service_config: &Self::ServiceConfig,
    ) -> Result<Router, Self::CreateRouterError>;

    async fn run(&self, service_config: &Self::ServiceConfig) -> Result<(), ServiceRunError> {
        let router = self
            .create_router(service_config)
            .await
            .map_err(|e| ServiceRunError::RouterCreationError(Box::new(e)))?;

        let endpoint = service_config.app_endpoint().map_or(
            Result::<String, ServiceRunError>::Err(ServiceRunError::MissingEndpointConfiguration),
            Result::<String, ServiceRunError>::Ok,
        )?;

        let addr = endpoint.parse::<SocketAddr>()?;
        tracing::info!("Starting on http://{}", addr);
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .with_graceful_shutdown(graceful_shutdown::shutdown_signal())
            .await
            .map_err(|e| ServiceRunError::ServeError(Box::new(e)))?;

        Ok(())
    }
}
