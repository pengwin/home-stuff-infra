use std::time::Duration;

use tokio::task::JoinHandle;

use service_core::service::Service;

use auth_service::{persistence::InmemoryPersistenceLayerFactory, AuthService};

use lazy_static::lazy_static;

use super::TestConfig;

fn start(test_config: &TestConfig) -> JoinHandle<()> {
    let cfg = test_config.to_service_configuration();
    let login = test_config.test_login.to_owned();
    let password = test_config.test_password.to_owned();
    tokio::task::spawn(async move {
        let cfg = cfg;
        let login = login;
        let password = password;

        let persistence_layer_factory =
            InmemoryPersistenceLayerFactory::new(&cfg, &login, &password);
        let service = AuthService::new(persistence_layer_factory);
        service.run(&cfg).await.expect("Unable to start server");
    })
}

pub struct TestContext {
    _guard: JoinHandle<()>,
    pub config: TestConfig,
}

lazy_static! {
    pub static ref TEST_CONTEXT: TestContext = {
        let config = TestConfig::default();
        let _guard = start(&config);

        // replace with healthcheck ping
        std::thread::sleep(Duration::from_millis(50));

        TestContext{
            _guard,
            config
        }
    };
}
