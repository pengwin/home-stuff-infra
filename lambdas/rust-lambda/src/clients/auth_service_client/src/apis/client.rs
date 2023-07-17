use std::sync::Arc;

use super::configuration::Configuration;

pub struct APIClient {
    auth_api: Box<dyn crate::apis::AuthApi>,
    healthcheck_api: Box<dyn crate::apis::HealthcheckApi>,
    users_api: Box<dyn crate::apis::UsersApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let arc = Arc::new(configuration);

        APIClient {
            auth_api: Box::new(crate::apis::AuthApiClient::new(arc.clone())),
            healthcheck_api: Box::new(crate::apis::HealthcheckApiClient::new(arc.clone())),
            users_api: Box::new(crate::apis::UsersApiClient::new(arc.clone())),
        }
    }

    pub fn auth_api(&self) -> &dyn crate::apis::AuthApi{
        self.auth_api.as_ref()
    }

    pub fn healthcheck_api(&self) -> &dyn crate::apis::HealthcheckApi{
        self.healthcheck_api.as_ref()
    }

    pub fn users_api(&self) -> &dyn crate::apis::UsersApi{
        self.users_api.as_ref()
    }

}