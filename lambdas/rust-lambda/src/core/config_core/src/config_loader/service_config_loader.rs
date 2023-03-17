use std::error::Error as StdError;

use async_trait::async_trait;

use crate::ServiceConfig;

#[async_trait]
pub trait ServiceConfigLoader {
    type Config: ServiceConfig;
    type Error: StdError + Send + Sync;

    async fn load(&self) -> Result<Self::Config, Self::Error>;
}
