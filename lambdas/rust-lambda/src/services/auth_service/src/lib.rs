mod config;
mod handlers;
pub mod persistence;
pub mod security;

use std::sync::Arc;

use axum::{async_trait, Router};
use handlers::{auth, users};
use persistence::{PersistenceLayer, PersistenceLayerFactory};
use service_core::handlers::healthcheck;
use service_core::service::Service;
use thiserror::Error;

use service_core::handlers::not_found::handler_404;
use service_core::{config_core, cors, open_api, tracing};

pub use self::config::AuthServiceConfig;
pub use handlers::api_doc::ApiDoc;

#[derive(Error, Debug)]
pub enum CreateRouterError {
    #[error("ConfigError: {0:?}")]
    ConfigError(#[from] config_core::ConfigError),
    #[error("PersistenceLayerFactoryError: {0:?}")]
    PersistenceLayerFactoryError(String),
    #[error("CreateCorsLayerError: {0:?}")]
    CreateCorsLayerError(#[from] cors::CreateCorsLayerError),
}

pub struct AuthService<P: PersistenceLayerFactory> {
    persistence_factory: P,
}

impl<P: PersistenceLayerFactory> AuthService<P> {
    pub fn new(persistence_factory: P) -> Self {
        Self {
            persistence_factory,
        }
    }

    async fn create_router_with_persistence(
        &self,
        app_config: &<Self as Service>::ServiceConfig,
        persistence_layer: Arc<dyn PersistenceLayer>,
    ) -> Result<Router, <Self as Service>::CreateRouterError> {
        let cors_layer = cors::create_cors_layer(&app_config.allowed_origin)?;

        let router: Router = Router::new()
            .merge(open_api::router::<ApiDoc>("http://localhost:3000"))
            .merge(users::users(persistence_layer.clone(), app_config))
            .merge(auth::auth(persistence_layer, app_config))
            .fallback(handler_404)
            .layer(tracing::trace_layer::create_layer(
                tracing::tracing::Span::current(),
            ))
            .layer(cors_layer)
            .merge(healthcheck::healthcheck());

        Ok(router)
    }
}

#[async_trait]
impl<P: PersistenceLayerFactory> Service for AuthService<P> {
    type ServiceConfig = AuthServiceConfig;
    type CreateRouterError = CreateRouterError;

    async fn create_router(
        &self,
        app_config: &Self::ServiceConfig,
    ) -> Result<Router, Self::CreateRouterError> {
        let parent = tracing::tracing::info_span!("auth_service");
        let _span_guard = parent.enter();

        let persistence_layer = self
            .persistence_factory
            .create::<AuthService<P>>(app_config)
            .await
            .map_err(|e| CreateRouterError::PersistenceLayerFactoryError(format!("{:?}", e)))?;
        self.create_router_with_persistence(app_config, persistence_layer)
            .await
    }
}
