use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("DotEnvError: {0:?}")]
    DotEnv(#[from] dotenvy::Error),
    #[error("ConfigError: {0:?}")]
    ConfigError(#[from] config::ConfigError),
}
