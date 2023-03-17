use std::path::PathBuf;

use auth_service::AuthServiceConfig;
use auth_service_client::apis::configuration::Configuration;
use config::{Config, Source, ValueKind};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct TestConfig {
    pub app_endpoint: String,
    pub test_login: String,
    pub test_password: String,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            app_endpoint: "127.0.0.1:3000".to_owned(),
            test_login: "test@admin".to_owned(),
            test_password: "test password".to_owned(),
        }
    }
}

impl TestConfig {
    pub fn to_api_configuration(&self) -> Configuration {
        let mut cfg = Configuration::default();
        cfg.base_path = format!("http://{}", &self.app_endpoint);
        cfg
    }

    pub fn to_credentials(&self) -> (String, String) {
        (self.test_login.clone(), self.test_password.clone())
    }

    pub fn to_service_configuration(&self) -> AuthServiceConfig {
        AuthServiceConfig {
            env: service_core::config_core::Env::Local,
            pepper: "pepper".to_owned(),
            secret: "secret".to_owned(),
            endpoint: Some(self.app_endpoint.to_owned()),
            jwt_expiration_sec: 5,
        }
    }
}
#[derive(Debug)]
struct DotEnvSource {
    path: String,
}

impl DotEnvSource {
    pub fn new(relative_path: &str) -> Self {
        let env_path = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}{}", env_path, relative_path);
        Self { path }
    }
}

impl Source for DotEnvSource {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new(Self {
            path: self.path.clone(),
        })
    }

    fn collect(&self) -> Result<config::Map<String, config::Value>, config::ConfigError> {
        let path = PathBuf::from(self.path.clone())
            .canonicalize()
            .map_err(|e| {
                config::ConfigError::Message(format!("Unable to canonicalize base path {:?}", e))
            })?;
        let path = match path.to_str() {
            Some(p) => Ok(p),
            None => Err(config::ConfigError::Message(format!(
                "Unable to cast base path to str"
            ))),
        }?;

        let iter = dotenvy::from_filename_iter(path)
            .map_err(|e| config::ConfigError::Message(format!("DotEnvError {:?}", e)))?;

        let mut map = config::Map::new();
        for item in iter {
            let (key, val) = item.map_err(|e| {
                config::ConfigError::Message(format!("Error with dotenv iter item {:?}", e))
            })?;

            let origin_str = &format!("DotEnvy::{}", key);
            let origin: Option<&String> = Some(&origin_str);

            let map_key = key.to_lowercase();

            // println!("{map_key}='{val}'");

            map.insert(map_key, config::Value::new(origin, ValueKind::String(val)));
        }

        Ok(map)
    }
}

#[allow(dead_code)]
pub fn load() -> TestConfig {
    let config = Config::builder()
        .add_source(DotEnvSource::new(concat!("/../../../", ".env")))
        .build()
        .expect("Config loading error");

    // println!("Config loaded");

    /*let map = config
    .try_deserialize::<std::collections::HashMap<String, String>>()
    .unwrap();

    for (k, v) in map {
        println!("{k}={v}");
    }

    todo!();*/

    let test_config: TestConfig = config
        .try_deserialize()
        .expect("Config Deserialization Error");

    test_config
}
