# Rust API client for auth-service-client

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.1.0
- Package version: 0.0.1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `auth-service-client` and add the following to `Cargo.toml` under `[dependencies]`:

```
auth-service-client = { path = "./auth-service-client" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AuthApi* | [**authorize**](docs/AuthApi.md#authorize) | **POST** /authorize | 
*AuthApi* | [**profile**](docs/AuthApi.md#profile) | **GET** /profile | 
*HealthcheckApi* | [**healthcheck**](docs/HealthcheckApi.md#healthcheck) | **GET** /healthcheck | 
*UsersApi* | [**add_user**](docs/UsersApi.md#add_user) | **PUT** /user | 
*UsersApi* | [**delete_user**](docs/UsersApi.md#delete_user) | **DELETE** /user/{user_id} | 
*UsersApi* | [**get_all_users**](docs/UsersApi.md#get_all_users) | **GET** /users | 
*UsersApi* | [**get_user**](docs/UsersApi.md#get_user) | **GET** /user/{user_id} | 


## Documentation For Models

 - [AddUserRequest](docs/AddUserRequest.md)
 - [AddUserResponse](docs/AddUserResponse.md)
 - [AuthErrorResponse](docs/AuthErrorResponse.md)
 - [AuthRequest](docs/AuthRequest.md)
 - [AuthSuccessResponse](docs/AuthSuccessResponse.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [GetAllResponse](docs/GetAllResponse.md)
 - [ProfileResponse](docs/ProfileResponse.md)
 - [SuccessResponse](docs/SuccessResponse.md)
 - [User](docs/User.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


