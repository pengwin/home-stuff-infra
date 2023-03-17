use lambda_core::LambdaHttpError;

use auth_service::AuthService;
use service_core::config_core::config_loader::ServiceConfigLoader;

#[cfg(feature = "lambda")]
#[tokio::main]
async fn main() -> Result<(), LambdaHttpError> {
    use auth_service::AuthServiceConfig;
    use lambda_core::lambda_service::LambdaService;
    use service_core::config_core::ServiceConfig;

    service_core::tracing::init();

    let persistence = auth_service::persistence::DynamoPersistenceLayerFactory::default();
    let app = AuthService::new(persistence);

    let config = AuthServiceConfig::default_loader().load().await?;
    app.run_lambda(&config).await?;

    Ok(())
}

#[cfg(feature = "test_run")]
#[tokio::main]
async fn main() -> Result<(), LambdaHttpError> {
    use service_core::config_core::config_loader::DotenvConfigLoader;
    use service_core::service::Service;

    service_core::tracing::init();

    let config_loader = DotenvConfigLoader::default();
    let config = config_loader.load().await?;

    let test_login = std::env::var("TEST_LOGIN")?;
    let test_password = std::env::var("TEST_PASSWORD")?;

    let persistence = auth_service::persistence::InmemoryPersistenceLayerFactory::new(
        &config,
        &test_login,
        &test_password,
    );
    let app = AuthService::new(persistence);

    app.run(&config).await?;
    Ok(())
}
