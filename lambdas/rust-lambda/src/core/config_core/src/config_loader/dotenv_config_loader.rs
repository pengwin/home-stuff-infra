use async_trait::async_trait;
use dotenvy::dotenv;

use crate::{ConfigError, ServiceConfig};

use super::{DefaultConfigLoader, ServiceConfigLoader};

pub struct DotenvConfigLoader<C: ServiceConfig> {
    inner: DefaultConfigLoader<C>,
}

impl<C: ServiceConfig> Default for DotenvConfigLoader<C> {
    fn default() -> Self {
        Self {
            inner: DefaultConfigLoader::default(),
        }
    }
}

#[async_trait]
impl<C: ServiceConfig> ServiceConfigLoader for DotenvConfigLoader<C> {
    type Config = C;
    type Error = ConfigError;

    async fn load(&self) -> Result<Self::Config, Self::Error> {
        dotenv()?;
        self.inner.load().await
    }
}
