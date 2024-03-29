/*
 * auth-service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthRequest {
    #[serde(rename = "login")]
    pub login: String,
    #[serde(rename = "password")]
    pub password: String,
}

impl AuthRequest {
    pub fn new(login: String, password: String) -> AuthRequest {
        AuthRequest {
            login,
            password,
        }
    }
}


