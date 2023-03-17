use aws_config::SdkConfig;

use config_core::{Env, ServiceConfig};

pub async fn load_aws_config<T: ServiceConfig>(app_config: &T) -> SdkConfig {
    let env = app_config.env();
    let is_local = &Env::Local == env || &Env::LocalStack == env;
    if !is_local {
        return aws_config::load_from_env().await;
    }

    super::local_stack_config::load_local_stack_config()
}
