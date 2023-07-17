use std::time::Duration;

use serde::Deserialize;

use service_core::config_core;

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct AuthServiceConfig {
    pub env: config_core::Env,
    pub pepper: String,
    pub secret: String,
    pub jwt_expiration_sec: u64,
    pub allowed_origin: String,
    pub endpoint: Option<String>,
}

impl config_core::ServiceConfig for AuthServiceConfig {
    fn env(&self) -> &config_core::Env {
        &self.env
    }

    fn env_prefix() -> &'static str {
        "APP"
    }

    fn app_endpoint(&self) -> Option<String> {
        self.endpoint.clone()
    }
}

impl service_core::auth::AuthConfig for AuthServiceConfig {
    fn secret(&self) -> &str {
        &self.secret
    }

    fn pepper(&self) -> &str {
        &self.pepper
    }

    fn jwt_expiration(&self) -> Duration {
        Duration::from_secs(self.jwt_expiration_sec)
    }
}
