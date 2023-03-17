mod common;

use auth_service_client::apis::users_api;

use crate::common::{add_user, authorize, del_user, setup};

#[tokio::test]
async fn healthcheck() {
    let cfg = setup::TEST_CONTEXT.config.clone();
    let api_config = cfg.to_api_configuration();

    crate::common::healthcheck(&api_config).await;
}

#[tokio::test]
async fn create_admin() {
    let mut api_config = setup::TEST_CONTEXT.config.to_api_configuration();

    let default_admin_token =
        authorize(&api_config, setup::TEST_CONTEXT.config.to_credentials()).await;
    println!("Default admin: {}", default_admin_token);
    api_config.bearer_access_token = Some(default_admin_token.clone());

    let local_admin = add_user(&api_config, true).await;

    let local_admin_token = authorize(&api_config, local_admin.to_credentials()).await;

    api_config.bearer_access_token = Some(local_admin_token);

    let _users = users_api::get_all_users(&api_config)
        .await
        .expect("getAllusers");

    // println!("Users: {:?}", users);

    api_config.bearer_access_token = Some(default_admin_token);

    del_user(&api_config, &local_admin.user_id).await;
}
