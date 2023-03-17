use std::time::Duration;

pub trait AuthConfig {
    fn secret(&self) -> &str;
    fn pepper(&self) -> &str;
    fn jwt_expiration(&self) -> Duration;
}
