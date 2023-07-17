/*
 * auth-service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: 
 * Generated by: https://openapi-generator.tech
 */

use std::sync::Arc;
use std::borrow::Borrow;
use std::pin::Pin;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct HealthcheckApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl HealthcheckApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> HealthcheckApiClient {
        HealthcheckApiClient {
            configuration,
        }
    }
}

pub trait HealthcheckApi {
    fn healthcheck(&self, ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
}

impl HealthcheckApi for HealthcheckApiClient {
    #[allow(unused_mut)]
    fn healthcheck(&self, ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/healthcheck".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

}