use service_core::{
    handlers::healthcheck::HealthCheckAddon, open_api::SecurityAddon, responses::ErrorResponse,
};
use utoipa::OpenApi;

use super::{auth, users};

#[derive(OpenApi)]
#[openapi(
    paths(
        users::get_all_users,
        users::add_user,
        users::get_user,
        users::delete_user,
        auth::authorize,
        auth::profile,
    ),
    components(schemas(
        users::User,
        users::GetAllResponse,
        users::SuccessResponse,
        ErrorResponse,
        users::AddUserRequest,
        users::AddUserResponse,
        auth::AuthRequest,
        auth::AuthSuccessResponse,
        auth::AuthErrorResponse,
        auth::ProfileResponse,
    )),
    modifiers(&SecurityAddon, &HealthCheckAddon),
)]
pub struct ApiDoc;
