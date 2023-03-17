use thiserror::Error;

use aws_sdk_dynamodb::{
    error::{
        DeleteItemError, GetItemError, PutItemError, QueryError, ScanError, TransactGetItemsError,
        TransactWriteItemsError,
    },
    types::SdkError,
};

#[derive(Error, Debug)]
pub enum PersistenceError {
    #[error("ScanError: {0:?}")]
    ScanError(#[from] SdkError<ScanError>),
    #[error("PutError: {0:?}")]
    PutError(#[from] SdkError<PutItemError>),
    #[error("GetError: {0:?}")]
    GetError(#[from] SdkError<GetItemError>),
    #[error("DeleteError: {0:?}")]
    DeleteError(#[from] SdkError<DeleteItemError>),
    #[error("TransactGetError: {0:?}")]
    TransactGetError(#[from] SdkError<TransactGetItemsError>),
    #[error("TransactWriteError: {0:?}")]
    TransactWriteError(#[from] SdkError<TransactWriteItemsError>),
    #[error("QueryError: {0:?}")]
    QueryError(#[from] SdkError<QueryError>),
    #[error("Transaction Get Output parsing error {0}")]
    TransactGetOutputParsingErrorError(String),
    #[error("SerdeError: {0:?}")]
    SerdeError(#[from] serde_dynamo::Error),
}
