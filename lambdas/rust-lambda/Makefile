.PHONY: auth-lambda-zip auth-service-client openapi-client build diagrams test auth-lambda-inmemory authload benchmark format lint

build:
	cargo build

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

openapi-client: auth-service-client

auth-service-client:
	docker compose run --rm -it openapi-generator generate \
    -i /local/src/lambdas/auth_lambda/auth-service.openapi.spec.json \
    -g rust \
	-p packageName=auth-service-client \
	-p packageVersion=0.0.1 \
	-p library=hyper \
    -o /local/src/clients/auth_service_client

# Load

load: authload

authload: 
	cargo run -p authload --release -- --host http://localhost:3000 --run-time 10s  --users 10 --report-file=./.artifacts/authload-report.html --no-reset-metrics

authload-local-stack: 
	cargo run -p authload --release -- --host http://27d2fb623b20f69254b78181d45aac10.lambda-url.us-east-1.localhost.localstack.cloud:4566/ --run-time 10s  --users 10 --report-file=./.artifacts/authload-report.html --no-reset-metrics

# Testing

test:
# --format json
	cargo test -- -Z unstable-options --report-time --nocapture 

# Diagrams

diagrams:
	docker compose run --rm -it diagrams python /diagrams/libraries.py

# service

format:
	cargo fmt --all -v

lint:
	cargo clippy