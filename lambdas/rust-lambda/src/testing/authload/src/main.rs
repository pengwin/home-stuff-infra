use goose::prelude::*;

use auth_service_client::models::{AuthRequest, AuthSuccessResponse};

async fn loadtest_index(user: &mut GooseUser) -> TransactionResult {
    let _goose_metrics = user.get("/healthcheck").await?;
    Ok(())
}

const TEST_LOGIN: &str = "test@test.com";
const TEST_PASSWORD: &str = "0w[pxTA%(V";

async fn loadtest_auth(user: &mut GooseUser) -> TransactionResult {
    let request = AuthRequest {
        login: TEST_LOGIN.to_owned(),
        password: TEST_PASSWORD.to_owned(),
    };

    let _auth_response = user.post_json("/authorize", &request).await?;

    //let token = auth_response.response?.json::<AuthSuccessResponse>().await?;

    Ok(())
}

struct Session {
    jwt_token: String,
}

async fn auth_user(user: &mut GooseUser) -> TransactionResult {
    let request = AuthRequest {
        login: TEST_LOGIN.to_owned(),
        password: TEST_PASSWORD.to_owned(),
    };

    let auth_response = user.post_json("/authorize", &request).await?;

    let res = auth_response.response.map_err(|e| Box::new(e.into()))?;

    let response = res
        .json::<AuthSuccessResponse>()
        .await
        .map_err(|e| Box::new(e.into()))?;

    user.set_session_data(Session {
        jwt_token: response.token,
    });

    Ok(())
}

async fn profile(user: &mut GooseUser) -> TransactionResult {
    // This will panic if the session is missing or if the session is not of the right type.
    // Use `get_session_data` to handle a missing session.
    let session = user.get_session_data_unchecked::<Session>();

    // Create a Reqwest RequestBuilder object and configure bearer authentication when making
    // a GET request for the index.
    let reqwest_request_builder = user
        .get_request_builder(&GooseMethod::Get, "/profile")?
        .bearer_auth(&session.jwt_token);

    // Add the manually created RequestBuilder and build a GooseRequest object.
    let goose_request = GooseRequest::builder()
        .set_request_builder(reqwest_request_builder)
        .name("/profile")
        .build();

    // Make the actual request.
    user.request(goose_request).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("HealthcheckAndAuth")
                .register_transaction(transaction!(loadtest_index))
                .register_transaction(transaction!(loadtest_auth)),
        )
        .register_scenario(
            scenario!("Profile")
                .register_transaction(transaction!(auth_user).set_on_start())
                .register_transaction(transaction!(profile)),
        )
        .execute()
        .await?;

    Ok(())
}
