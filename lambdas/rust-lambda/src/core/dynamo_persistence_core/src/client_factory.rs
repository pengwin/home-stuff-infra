use aws_sdk_dynamodb::Client;

pub struct ClientFactory {
    client: Client,
}

impl ClientFactory {
    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
        Self {
            client: Client::new(sdk_config),
        }
    }

    pub fn create_client(&self) -> Client {
        self.client.clone()
    }
}
