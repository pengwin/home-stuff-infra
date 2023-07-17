use aws_config::SdkConfig;

use config_core::{Env, ServiceConfig};

pub async fn load_aws_config<T: ServiceConfig>(app_config: &T) -> SdkConfig {
    let is_local = match *app_config.env() {
        Env::Local => true,
        Env::LocalStack => true,
        Env::ProdEU => false,
    };
    if !is_local {
        return aws_config::load_from_env().await;
    }

    super::local_stack_config::load_local_stack_config()
}
