use axum::async_trait;
use dynamo_persistence_core::{
    transaction::{TransactionExecutor, TransactionWrite},
    ClientFactory, PersistenceError,
};

use crate::persistence::{UserCredDbModel, UserDbModel, UserTxRepository};

pub struct DynamoUserTxRepository {
    tx_executor: TransactionExecutor,
}

impl DynamoUserTxRepository {
    #[allow(dead_code)]
    pub fn new(client_factory: &ClientFactory) -> Self {
        Self {
            tx_executor: TransactionExecutor::new(client_factory),
        }
    }
}

#[async_trait]
impl UserTxRepository for DynamoUserTxRepository {
    async fn save(
        &self,
        user: &UserDbModel,
        user_cred: &UserCredDbModel,
    ) -> Result<(), PersistenceError> {
        let tx = TransactionWrite::builder()
            .put(user)?
            .put(user_cred)?
            .build();

        self.tx_executor.send_write(tx).await
    }

    async fn delete(&self, user_uuid: &uuid::Uuid, email: &str) -> Result<(), PersistenceError> {
        let tx = TransactionWrite::builder()
            .delete::<UserDbModel, String>(user_uuid.to_string())
            .delete::<UserCredDbModel, String>(email.to_owned())
            .build();

        self.tx_executor.send_write(tx).await
    }
}
