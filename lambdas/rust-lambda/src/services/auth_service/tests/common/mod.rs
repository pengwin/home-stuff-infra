mod config;
pub mod setup;

use anyhow::Context;
use auth_service_client::{
    apis::client::APIClient,
    models::{AddUserRequest, AuthRequest, GetAllResponse},
};
use uuid::Uuid;

pub use self::config::load as load_config;
pub use self::config::TestConfig;

pub async fn healthcheck(client: &APIClient) -> anyhow::Result<()> {
    client
        .healthcheck_api()
        .healthcheck()
        .await
        .context("Healthcheck error")?;
    Ok(())
}

pub async fn authorize(
    client: &APIClient,
    credentials: (String, String),
) -> anyhow::Result<String> {
    let token = client
        .auth_api()
        .authorize(AuthRequest {
            login: credentials.0,
            password: credentials.1,
        })
        .await
        .context("Auth error")?;
    Ok(token.token)
}

pub struct CreatedUser {
    pub user_id: Uuid,
    pub email: String,
    pub password: String,
}

impl CreatedUser {
    pub fn to_credentials(&self) -> (String, String) {
        (self.email.clone(), self.password.clone())
    }
}

pub async fn add_user(client: &APIClient, is_admin: bool) -> anyhow::Result<CreatedUser> {
    let id = uuid::Uuid::new_v4().to_string();
    let password = uuid::Uuid::new_v4().to_string().replace("-", "");

    let username = format!("test-admin-{}", id);
    let email = format!("{}@test.com", username);

    let added_user = client
        .users_api()
        .add_user(AddUserRequest {
            admin: is_admin,
            email: email.clone(),
            password: password.clone(),
            username,
        })
        .await
        .context("Add User Error")?;

    Ok(CreatedUser {
        user_id: added_user.user.user_id,
        email: email.clone(),
        password: password.clone(),
    })
}

pub async fn del_user(client: &APIClient, user_id: &Uuid) -> anyhow::Result<()> {
    client
        .users_api()
        .delete_user(&user_id.to_string())
        .await
        .context("Delete User Error")?;
    Ok(())
}

pub async fn get_all_users(client: &APIClient) -> anyhow::Result<GetAllResponse> {
    let users = client
        .users_api()
        .get_all_users()
        .await
        .context("Get all Users Error")?;

    Ok(users)
}
