/*
 * auth-service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact:
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetAllResponse {
    #[serde(rename = "users")]
    pub users: Vec<crate::models::User>,
}

impl GetAllResponse {
    pub fn new(users: Vec<crate::models::User>) -> GetAllResponse {
        GetAllResponse { users }
    }
}
