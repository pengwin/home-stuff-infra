use std::marker::PhantomData;

use async_trait::async_trait;
use config::Config;

use crate::{ConfigError, ServiceConfig};

use super::ServiceConfigLoader;

pub struct DefaultConfigLoader<C: ServiceConfig> {
    _c: PhantomData<C>,
}

impl<C: ServiceConfig> Default for DefaultConfigLoader<C> {
    fn default() -> Self {
        Self {
            _c: PhantomData::default(),
        }
    }
}

#[async_trait]
impl<C: ServiceConfig> ServiceConfigLoader for DefaultConfigLoader<C> {
    type Config = C;
    type Error = ConfigError;

    async fn load(&self) -> Result<Self::Config, Self::Error> {
        let config = Config::builder()
            .add_source(
                config::Environment::with_prefix(Self::Config::env_prefix()).try_parsing(true),
            )
            .build()?;

        let app_config: Self::Config = config.try_deserialize()?;

        Ok(app_config)
    }
}
