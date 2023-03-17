use aws_config::SdkConfig;
use aws_credential_types::provider::{ProvideCredentials, SharedCredentialsProvider};
use aws_credential_types::Credentials;
use aws_types::region::Region;

const DEFAULT_ENDPOINT: &str = "http://localhost:4566";
const DEFAULT_REGION: Region = Region::from_static("us-east-1");

fn load_local_stack_endpoint() -> Option<String> {
    let hostname_os = std::env::var_os("LOCALSTACK_HOSTNAME")?;
    let port_os = std::env::var_os("EDGE_PORT")?;

    let hostname = hostname_os.to_str()?;
    let port = port_os.to_str()?;

    Some(format!("http://{}:{}", hostname, port))
}

fn default_credentials() -> impl ProvideCredentials {
    Credentials::new("test", "test", None, None, "example")
}

fn default_credentials_provider() -> SharedCredentialsProvider {
    SharedCredentialsProvider::new(default_credentials())
}

pub fn load_local_stack_config() -> SdkConfig {
    let endpoint = if let Some(endpoint) = load_local_stack_endpoint() {
        endpoint
    } else {
        DEFAULT_ENDPOINT.to_owned()
    };

    aws_config::SdkConfig::builder()
        .region(Some(DEFAULT_REGION))
        .endpoint_url(endpoint)
        .credentials_provider(default_credentials_provider())
        .build()
}
