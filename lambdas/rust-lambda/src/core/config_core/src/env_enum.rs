use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub enum Env {
    // Running locally as standalone application
    #[default]
    Local,
    // Running locally inside local-stack as lambda
    LocalStack,
    // Running on eu-central-1
    ProdEU,
}
