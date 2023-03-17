mod config;

pub use self::config::*;

async fn authorize(configuration: &Configuration, login: &str, pass: &str) -> String {
    superauth_api::authorize(
        configuration,
        AuthRequest {
            login: login.to_owned(),
            password: pass.to_owned(),
        },
    )
    .await
    .expect("Auth error")
    .token
}

struct CreatedUser {
    email: String,
    password: String,
}

async fn add_user(configuration: &Configuration, is_admin: bool) -> CreatedUser {
    let id = uuid::Uuid::new_v4().to_string();
    let password = uuid::Uuid::new_v4().to_string().replace("-", "");

    let username = format!("test-admin-{}", id);
    let email = format!("{}@test.com", username);

    let result = CreatedUser {
        email: email.clone(),
        password: password.clone(),
    };

    superusers_api::add_user(
        configuration,
        AddUserRequest {
            admin: is_admin,
            email,
            password,
            username,
        },
    )
    .await
    .expect("Add Admin Error");

    result
}