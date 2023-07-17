use anyhow::Context;
use auth_service_client::apis::client::APIClient;

use service_core::service::Service;

use auth_service::{persistence::InmemoryPersistenceLayerFactory, AuthService};

use super::TestConfig;

pub struct TestContext {
    app: axum::Router,
    config: TestConfig,
}

impl TestContext {
    pub async fn start() -> anyhow::Result<Self> {
        let test_config = TestConfig::default();
        let cfg = test_config.to_service_configuration();
        let login = test_config.test_login.to_owned();
        let password = test_config.test_password.to_owned();

        let persistence_layer_factory =
            InmemoryPersistenceLayerFactory::new(&cfg, &login, &password);
        let service = AuthService::new(persistence_layer_factory);
        let app = service
            .create_router(&cfg)
            .await
            .context("Unable to create router")?;

        Ok(TestContext {
            app,
            config: test_config.clone(),
        })
    }

    pub fn get_credentials(&self) -> (String, String) {
        self.config.to_credentials()
    }

    pub fn create_api_client_with_auth(&self, auth_token: String) -> APIClient {
        self.create_api_client_with_token(Some(auth_token))
    }

    pub fn create_api_client_without_auth(&self) -> APIClient {
        self.create_api_client_with_token(None)
    }

    fn create_api_client_with_token(&self, auth_token: Option<String>) -> APIClient {
        let configuration = self
            .config
            .to_api_configuration(self.app.clone(), auth_token);
        APIClient::new(configuration)
    }
}
