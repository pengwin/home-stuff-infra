/*
 * rust-lambda
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: pengwin4@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommandResponse {
    #[serde(rename = "command")]
    pub command: String,
}

impl CommandResponse {
    pub fn new(command: String) -> CommandResponse {
        CommandResponse {
            command,
        }
    }
}


