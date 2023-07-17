use std::{str::FromStr, sync::Arc};

use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, put},
    Extension, Json, Router,
};
use chrono::{DateTime, Utc};
use dynamo_persistence_core::PersistenceError;
use serde::{Deserialize, Serialize};

use service_core::{
    auth::{AdminClaims, AuthConfig, JwtAuthState},
    responses::ErrorResponse,
};
use validator::{Validate, ValidationErrors};

use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct User {
    pub user_id: uuid::Uuid,
    pub username: String,
    pub is_admin: bool,
    pub created_utc: DateTime<Utc>,
    pub updated_utc: DateTime<Utc>,
}

impl From<&UserDbModel> for User {
    fn from(u: &UserDbModel) -> Self {
        Self {
            user_id: u.user_id,
            username: u.username.clone(),
            is_admin: u.is_admin,
            created_utc: u.created_utc,
            updated_utc: u.updated_utc,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct SuccessResponse {
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct GetAllResponse {
    pub users: Vec<User>,
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, body = GetAllResponse),
        (status = 500, body = ErrorResponse)
    ),
    security(
        ("token" = [])
    )
)]
#[debug_handler]
async fn get_all_users(
    _admin: AdminClaims,
    State(state): State<ArcState>,
) -> Result<Json<GetAllResponse>, UsersApiError> {
    let users: Vec<User> = state
        .user_repository
        .get_all()
        .await?
        .iter()
        .map(User::from)
        .collect();

    Ok(Json(GetAllResponse { users }))
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
pub struct AddUserRequest {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(length(min = 10))]
    pub password: String,
    #[validate(email)]
    pub email: String,
    pub admin: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct AddUserResponse {
    pub user: User,
}

#[utoipa::path(
    put,
    path = "/user",
    request_body = AddUserRequest,
    responses(
        (status = 200, body = AddUserResponse),
        (status = 400, body = ErrorResponse),
        (status = 500, body = ErrorResponse),
    ),
    security(
        ("token" = [])
    )
)]
#[debug_handler]
pub async fn add_user(
    admin: AdminClaims,
    State(state): State<ArcState>,
    Json(payload): Json<AddUserRequest>,
) -> Result<Json<AddUserResponse>, UsersApiError> {
    payload.validate()?;

    let (user, user_cred) = create_user(
        &payload.username,
        &payload.email,
        &payload.password,
        payload.admin,
        Some(*admin.user_id()),
        &state.password_hasher,
    );

    state.user_tx_repository.save(&user, &user_cred).await?;

    Ok(Json(AddUserResponse {
        user: User::from(&user),
    }))
}

pub fn create_user(
    username: &str,
    email: &str,
    password: &str,
    is_admin: bool,
    created_by: Option<uuid::Uuid>,
    password_hasher: &PasswordHasher,
) -> (UserDbModel, UserCredDbModel) {
    let salt = uuid::Uuid::new_v4().to_string();
    let user_id = uuid::Uuid::new_v4();
    let password_hash = password_hasher.hash_password(password, &salt);

    let timestamp = Utc::now();

    let user = UserDbModel {
        user_id,
        username: username.to_owned(),
        is_admin,
        created_by,
        created_utc: timestamp,
        updated_utc: timestamp,
    };

    let user_cred = UserCredDbModel {
        user_id,
        email: email.to_owned(),
        password_hash,
        salt,
        created_by,
        created_utc: timestamp,
        updated_utc: timestamp,
    };

    (user, user_cred)
}

#[utoipa::path(
    get,
    path = "/user/{user_id}",
    params(
        ("user_id" = String, Path, )
    ),
    request_body = User,
    responses(
        (status = 200, body = User),
        (status = 404, body = ErrorResponse),
        (status = 400, body = ErrorResponse),
        (status = 500, body = ErrorResponse),
    ),
    security(
        ("token" = [])
    )
)]
#[debug_handler]
pub async fn get_user(
    _admin: AdminClaims,
    State(state): State<ArcState>,
    Path(user_id): Path<String>,
) -> Result<Json<User>, UsersApiError> {
    let user_uuid = uuid::Uuid::from_str(&user_id)?;

    let user = state.user_repository.get_user(&user_uuid).await?;

    match user {
        Some(user) => Ok(Json(User::from(&user))),
        None => Err(UsersApiError::UserNotFound),
    }
}

#[utoipa::path(
    delete,
    path = "/user/{user_id}",
    params(
        ("user_id" = String, Path, )
    ),
    responses(
        (status = 200, body = SuccessResponse),
        (status = 400, body = ErrorResponse),
        (status = 403, body = ErrorResponse),
        (status = 500, body = ErrorResponse),
    ),
    security(
        ("token" = [])
    )
)]
#[debug_handler]
pub async fn delete_user(
    admin: AdminClaims,
    State(state): State<ArcState>,
    Path(user_id): Path<String>,
) -> Result<Json<SuccessResponse>, UsersApiError> {
    let user_uuid = uuid::Uuid::from_str(&user_id)?;

    let user = state.user_repository.get_user(&user_uuid).await?;

    let user = match user {
        Some(u) => Ok(u),
        _ => Err(UsersApiError::UserNotFound),
    }?;

    let cred = state
        .user_cred_repository
        .get_cred_by_user_id(&user_uuid)
        .await?;

    let cred = match cred.len() {
        0 => Err(UsersApiError::CredentialsError(
            "Credentials not found".to_string(),
        )),
        1 => Ok(&cred[0]),
        _ => Err(UsersApiError::CredentialsError(format!(
            "Unexpected multiple credentials {}",
            cred.len()
        ))),
    }?;

    match (
        user.created_by.map(|e| admin.is_admin(&e)),
        cred.created_by.map(|e| admin.is_admin(&e)),
    ) {
        (Some(true), Some(true)) => Ok(()),
        _ => Err(UsersApiError::NotAllowed),
    }?;

    state
        .user_tx_repository
        .delete(&user_uuid, &cred.email)
        .await?;

    Ok(Json(SuccessResponse {
        message: "Deleted".to_string(),
    }))
}

use thiserror::Error;

use crate::{
    config::AuthServiceConfig,
    persistence::{
        PersistenceLayer, UserCredDbModel, UserCredRepository, UserDbModel, UserRepository,
        UserTxRepository,
    },
    security::PasswordHasher,
};

type ArcState = Arc<UsersState>;

#[derive(Clone)]
pub struct UsersState {
    user_tx_repository: Arc<dyn UserTxRepository>,
    user_repository: Arc<dyn UserRepository>,
    user_cred_repository: Arc<dyn UserCredRepository>,
    password_hasher: Arc<PasswordHasher>,
}

pub fn users(persistence: Arc<dyn PersistenceLayer>, config: &AuthServiceConfig) -> Router {
    let password_hasher = Arc::new(PasswordHasher::new(config.pepper()));
    let auth_state = Arc::new(JwtAuthState::new(config));
    Router::new()
        .route("/users", get(get_all_users))
        .route("/user", put(add_user))
        .route("/user/:user_id", get(get_user))
        .route("/user/:user_id", delete(delete_user))
        .layer(Extension(persistence.user_provider()))
        .layer(Extension(auth_state))
        .with_state(Arc::new(UsersState {
            user_repository: persistence.user_repository(),
            user_cred_repository: persistence.user_cred_repository(),
            user_tx_repository: persistence.user_tx_repository(),
            password_hasher,
        }))
}

#[derive(Error, Debug)]
pub enum UsersApiError {
    #[error("UuidError: {0:?}")]
    UuidError(#[from] uuid::Error),
    #[error("UserNotFound")]
    UserNotFound,
    #[error("NotAllowed")]
    NotAllowed,
    #[error("CredentialsError: {0:?}")]
    CredentialsError(String),
    #[error("ValidationError: {0:?}")]
    ValidationError(#[from] ValidationErrors),
    #[error("PersistenceError: {0:?}")]
    PersistenceError(#[from] PersistenceError),
}

impl IntoResponse for UsersApiError {
    fn into_response(self) -> Response {
        match self {
            UsersApiError::UuidError(ue) => {
                ErrorResponse::new(StatusCode::BAD_REQUEST, &format!("UUID error: {:?}", ue))
                    .into_response()
            }
            UsersApiError::PersistenceError(pe) => ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("SDK error: {:?}", pe),
            )
            .into_response(),
            UsersApiError::UserNotFound => {
                ErrorResponse::new(StatusCode::NOT_FOUND, "User not found").into_response()
            }
            UsersApiError::NotAllowed => {
                ErrorResponse::new(StatusCode::FORBIDDEN, "Not Allowed for this user")
                    .into_response()
            }
            UsersApiError::ValidationError(ve) => ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                &format!("Invalid request: {:?}", ve),
            )
            .into_response(),
            UsersApiError::CredentialsError(ce) => ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Credentials error: {:?}", ce),
            )
            .into_response(),
        }
    }
}
