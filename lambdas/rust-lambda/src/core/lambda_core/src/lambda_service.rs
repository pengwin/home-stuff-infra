use axum::Router;
use lambda_http::{tower::ServiceBuilder, Error};

use service_core::{
    async_trait,
    service::{Service, ServiceRunError},
};

#[async_trait]
pub trait LambdaService: Service {
    async fn run_lambda(&self, service_config: &Self::ServiceConfig) -> Result<(), Error> {
        match self.create_lambda_router(service_config).await {
            Ok(router) => {
                let app = ServiceBuilder::new()
                    .layer(axum_aws_lambda::LambdaLayer::default())
                    .service(router);

                lambda_http::run(app).await
            }
            Err(e) => super::broken_service::broken_service(e).await,
        }
    }

    async fn create_lambda_router(
        &self,
        service_config: &Self::ServiceConfig,
    ) -> Result<Router, ServiceRunError> {
        let router = self
            .create_router(service_config)
            .await
            .map_err(|e| ServiceRunError::RouterCreationError(Box::new(e)))?;

        Ok(router)
    }
}

impl<T: Service> LambdaService for T {}
