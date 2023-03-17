# home-stuff-infra

## Prerequisites

0. Nodejs, pnpm.
1. Terraform CLI - *win*:  ```choco install terraform```
2. Make ```choco install make```
3. Linux or WSL with Rust and Cargo Lambda
    1. Rust  ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
    2. Cargo Lambda ```cargo install cargo-lambda```


## How to deploy to local stack

1. Run local stack  ```make run-local-stack```
2. Deploy  ```make deploy-local-stack```

