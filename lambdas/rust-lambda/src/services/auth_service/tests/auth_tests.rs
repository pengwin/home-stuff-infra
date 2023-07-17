mod common;

use crate::common::{add_user, authorize, del_user, get_all_users, setup};

#[tokio::test]
async fn healthcheck() -> anyhow::Result<()> {
    let context = setup::TestContext::start().await?;
    let client = context.create_api_client_without_auth();
    crate::common::healthcheck(&client).await?;
    Ok(())
}

#[tokio::test]
async fn create_admin() -> anyhow::Result<()> {
    let context = setup::TestContext::start().await?;

    let default_admin_token = authorize(
        &context.create_api_client_without_auth(),
        context.get_credentials(),
    )
    .await?;
    println!("Default admin: {}", default_admin_token);
    let client_with_default_admin =
        context.create_api_client_with_auth(default_admin_token.clone());

    let local_admin = add_user(&client_with_default_admin, true).await?;
    let local_admin_token =
        authorize(&client_with_default_admin, local_admin.to_credentials()).await?;

    let client_with_local_admin = context.create_api_client_with_auth(local_admin_token.clone());

    let users = get_all_users(&client_with_local_admin).await?;

    users
        .users
        .iter()
        .find(|u| u.user_id == local_admin.user_id)
        .ok_or(anyhow::anyhow!("Local admin not found"))?;

    del_user(&client_with_default_admin, &local_admin.user_id).await?;

    Ok(())
}
