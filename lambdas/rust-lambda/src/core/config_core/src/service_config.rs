use serde::de::DeserializeOwned;

use crate::{config_loader::DefaultConfigLoader, env_enum::Env};

pub trait ServiceConfig: Default + DeserializeOwned + PartialEq + Eq + Send + Sync {
    fn app_endpoint(&self) -> Option<String>;
    fn env(&self) -> &Env;
    fn env_prefix() -> &'static str;

    fn default_loader() -> DefaultConfigLoader<Self> {
        DefaultConfigLoader::default()
    }
}
