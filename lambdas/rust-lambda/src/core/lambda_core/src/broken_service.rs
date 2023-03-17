use lambda_http::{run, service_fn, Body, Error, Request, Response};

pub async fn broken_service<E: std::error::Error + Send + Sync>(e: E) -> Result<(), Error> {
    let function_handler = move |_event: Request| {
        let body = serde_json::json!({ "error": format!("Error: {:?}", e) }).to_string();

        async move {
            let resp: Response<Body> = Response::builder()
                .status(500)
                .header("content-type", "application/json")
                .body(body.into())
                .map_err(Box::new)?;
            Result::<Response<Body>, Error>::Ok(resp)
        }
    };

    run(service_fn(function_handler)).await
}
