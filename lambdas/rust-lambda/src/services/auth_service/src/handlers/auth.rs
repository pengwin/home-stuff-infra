use std::sync::Arc;

use axum::{
    debug_handler,
    extract::State,
    routing::{get, post},
    Extension, Json, Router,
};

use serde::{Deserialize, Serialize};

use service_core::auth::{AuthConfig, AuthError, Claims, JwtAuthState};
use utoipa::ToSchema;

use crate::{
    config::AuthServiceConfig,
    persistence::{PersistenceLayer, UserCredDbModel, UserCredRepository, UserRepository},
    security::PasswordHasher,
};

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct AuthRequest {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct AuthSuccessResponse {
    pub token: String,
}

pub type AuthErrorResponse = service_core::auth::AuthErrorResponse;

#[utoipa::path(
    post,
    path = "/authorize",
    request_body = AuthRequest,
    responses(
        (status = 200, body = AuthSuccessResponse),
        (status = 500, body = AuthErrorResponse),
    )
)]
#[debug_handler]
async fn authorize(
    State(state): State<ArcState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<AuthSuccessResponse>, AuthError> {
    let res = state
        .user_cred_repository
        .get_cred(&payload.login)
        .await
        .map_err(|e| AuthError::PersistenceError(e.to_string()))?;

    match res {
        None => Err(AuthError::WrongCredentials),
        Some(cred) => check_credentials(&state, cred, &payload.password),
    }
}

fn check_credentials(
    state: &HandlerState,
    cred: UserCredDbModel,
    password: &str,
) -> Result<Json<AuthSuccessResponse>, AuthError> {
    let hash = state.password_hasher.hash_password(password, &cred.salt);
    if hash != cred.password_hash {
        return Err(AuthError::WrongCredentials);
    }

    let token = state.auth_state.create_token(&cred.user_id)?;
    Ok(Json(AuthSuccessResponse { token }))
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct ProfileResponse {
    pub username: String,
}

#[utoipa::path(
    get,
    path = "/profile",
    responses(
        (status = 200, body = ProfileResponse),
        (status = 403, body = AuthErrorResponse),
        (status = 404, body = AuthErrorResponse),
    ),
    security(
        ("token" = [])
    )
)]
#[debug_handler]
async fn profile(
    State(state): State<ArcState>,
    claims: Claims,
) -> Result<Json<ProfileResponse>, AuthError> {
    let user = state
        .user_repository
        .get_user(claims.user_id())
        .await
        .map_err(|e| AuthError::PersistenceError(e.to_string()))?;

    match user {
        Some(u) => Ok(Json(ProfileResponse {
            username: u.username,
        })),
        None => Err(AuthError::WrongCredentials),
    }
}

type ArcState = Arc<HandlerState>;

#[derive(Clone)]
struct HandlerState {
    user_cred_repository: Arc<dyn UserCredRepository>,
    user_repository: Arc<dyn UserRepository>,
    auth_state: Arc<JwtAuthState>,
    password_hasher: Arc<PasswordHasher>,
}

pub fn auth(persistence: Arc<dyn PersistenceLayer>, config: &AuthServiceConfig) -> Router {
    let auth_state = Arc::new(JwtAuthState::new(config));
    let password_hasher = Arc::new(PasswordHasher::new(config.pepper()));
    Router::new()
        .route("/authorize", post(authorize))
        .route("/profile", get(profile))
        .layer(Extension(auth_state.clone()))
        .with_state(Arc::new(HandlerState {
            user_cred_repository: persistence.user_cred_repository(),
            user_repository: persistence.user_repository(),
            auth_state,
            password_hasher,
        }))
}
