use config::Config;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub pepper: String,
    pub secret: String,
    pub env: String,
    pub is_lambda: bool,
    pub login: String,
    pub password: String,
}

pub fn load() -> Result<AppConfig, config::ConfigError> {
    let is_lambda = false;
    if !is_lambda {
        let config = Config::builder()
            .set_default(stringify!(is_lambda), is_lambda)?
            .add_source(config::File::with_name("C:\\coding\\pet\\home-stuff-infra\\lambdas\\rust-lambda\\test.json"))
            .build()?;

        let app_config: AppConfig = config.try_deserialize()?;

        return Ok(app_config);
    }

    let config = Config::builder()
        .set_default(stringify!(is_lambda), is_lambda)?
        .add_source(
            config::Environment::with_prefix("APP")
                .try_parsing(true)
                .separator("_"),
        )
        .build()?;

    let app_config: AppConfig = config.try_deserialize()?;

    Ok(app_config)
}
