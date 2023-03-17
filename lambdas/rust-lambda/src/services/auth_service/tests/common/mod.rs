mod config;
pub mod setup;

use auth_service_client::{
    apis::{auth_api, configuration::Configuration, healthcheck_api, users_api},
    models::{AddUserRequest, AuthRequest},
};
use uuid::Uuid;

pub use self::config::load as load_config;
pub use self::config::TestConfig;

pub async fn healthcheck(configuration: &Configuration) {
    healthcheck_api::healthcheck(configuration)
        .await
        .expect("HealthCheckError")
}

pub async fn authorize(configuration: &Configuration, credentials: (String, String)) -> String {
    auth_api::authorize(
        configuration,
        AuthRequest {
            login: credentials.0,
            password: credentials.1,
        },
    )
    .await
    .expect("Auth error")
    .token
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

pub async fn add_user(configuration: &Configuration, is_admin: bool) -> CreatedUser {
    let id = uuid::Uuid::new_v4().to_string();
    let password = uuid::Uuid::new_v4().to_string().replace("-", "");

    let username = format!("test-admin-{}", id);
    let email = format!("{}@test.com", username);

    let added_user = users_api::add_user(
        configuration,
        AddUserRequest {
            admin: is_admin,
            email: email.clone(),
            password: password.clone(),
            username,
        },
    )
    .await
    .expect("Add User Error");

    CreatedUser {
        user_id: added_user.user.user_id,
        email: email.clone(),
        password: password.clone(),
    }
}

pub async fn del_user(configuration: &Configuration, user_id: &Uuid) {
    users_api::delete_user(configuration, &user_id.to_string())
        .await
        .expect("Delete User Error");
}
