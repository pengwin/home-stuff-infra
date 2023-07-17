.PHONY: auth-lambda-zip openapi-clients auth-service-client auth-service-rs-client auth-service-ts-client openapi-client build diagrams test auth-lambda-inmemory authload benchmark format lint clean

build:
	cargo build

clean:
	cargo clean -v

# ZIP

zip: auth-lambda-zip

auth-lambda-zip:
	cargo lambda build -p auth-lambda --release --no-default-features --features "lambda" --output-format zip

# benchmark

benchmark: 
	cargo bench

# Test run

auth-lambda-test-run:
	cargo run -p auth-lambda --no-default-features --features "test_run" #--release

# Clients

openapi-clients: auth-service-client

auth-service-client: auth-service-ts-client auth-service-rs-client

auth-service-rs-client:
	docker compose run --rm -it openapi-generator generate \
    -i /local/src/lambdas/auth_lambda/auth-service.openapi.spec.json \
    -g rust \
	--library hyper \
	--template-dir /local/src/clients/rust_templates \
	-p packageName=auth-service-client \
	-p packageVersion=0.0.1 \
    -o /local/src/clients/auth_service_client

auth-service-ts-client:
	docker compose run --rm -it openapi-generator generate \
    -i /local/src/lambdas/auth_lambda/auth-service.openapi.spec.json \
    -g typescript-fetch \
	-p packageName=auth-service-client \
	-p packageVersion=0.0.1 \
	--additional-properties=supportsES6=true,npmVersion=6.9.0,typescriptThreePlus=true,withInterfaces=true \
    -o /front/src/api/auth-service-client

# Load

load: authload

authload: 
	cargo run -p authload --release -- --host http://localhost:3000 --run-time 10s  --users 10 --report-file=./.artifacts/authload-report.html --no-reset-metrics

authload-local-stack: 
	cargo run -p authload --release -- --host http://27d2fb623b20f69254b78181d45aac10.lambda-url.us-east-1.localhost.localstack.cloud:4566/ --run-time 10s  --users 10 --report-file=./.artifacts/authload-report.html --no-reset-metrics

# Testing

test:
# --format json
# -- -Z unstable-options --report-time --nocapture 
	cargo test 

# Diagrams

diagrams:
	docker compose run --rm -it diagrams python /diagrams/libraries.py

# service

format:
	cargo fmt --all -v

lint:
	cargo clippy