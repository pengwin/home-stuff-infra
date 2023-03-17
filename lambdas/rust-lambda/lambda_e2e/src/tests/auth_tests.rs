mod common;

use lambda_client::{
    apis::{configuration::Configuration, superauth_api, superusers_api},
    models::{AddUserRequest, AuthRequest},
};

use lambda_client::apis::configuration::Configuration;

use lambda_client::apis::superusers_api;

#[tokio::test]
async fn create_admin() {
    let cfg = crate::config::load().expect("Config error");
    let mut api_config = Configuration::default();
    api_config.base_path = "http://localhost:3000".to_owned();

    let default_admin_token = crate::authorize(&api_config, &cfg.login, &cfg.password).await;
    println!("Default admin: {}", default_admin_token);
    api_config.bearer_access_token = Some(default_admin_token);

    let local_admin = crate::add_user(&api_config, true).await;

    let local_admin_token =
        crate::authorize(&api_config, &local_admin.email, &local_admin.password).await;

    api_config.bearer_access_token = Some(local_admin_token);

    let users = superusers_api::get_all_users(&api_config)
        .await
        .expect("getAllusers");

    println!("Users: {:?}", users)
}
