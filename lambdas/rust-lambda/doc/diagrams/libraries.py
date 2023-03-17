from diagrams import Cluster, Diagram
from diagrams.programming.language import Rust
from diagrams.aws.compute import Lambda
from diagrams.aws.database import Dynamodb

with Diagram("Backend libraries", show=False, outformat=["png"], filename="/diagrams/libraries"):

    with Cluster("External"):
        axum = Rust("axum")

    with Cluster("Cfg External"):
        dotenvy = Rust("dotenvy")
        config = Rust("config")

    with Cluster("AWS External"):
        lambda_runtime = Rust("lambda_runtime")
        lambda_http = Rust("lambda_http")
        aws_config = Rust("aws-config")

    with Cluster("AWS Dynamo External"):
        aws_sdk_dynamodb = Rust("aws-sdk-dynamodb")
        serde_dynamo = Rust("serde_dynamo")
        
    lambda_core = Rust("lambda-core")
    service_core = Rust("service-core")
    config_core = Rust("config-core")
    aws_config_core = Rust("aws-config-core")
    dynamo_persistence_core = Rust("dynamo-persistence-core")

    auth_service = Rust("auth-service")
    auth_lambda = Lambda("auth-lambda")
    users_db = Dynamodb("users")

    [dotenvy, config] >> config_core
    [config_core, axum] >> service_core

    [aws_config] >> aws_config_core
    [serde_dynamo, aws_sdk_dynamodb] >> dynamo_persistence_core
    [config_core, lambda_http, lambda_runtime, aws_config] >> lambda_core
    
    [config_core, service_core, dynamo_persistence_core, aws_config_core] >> auth_service 

    auth_service >> auth_lambda >> users_db

