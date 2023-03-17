use aws_sdk_dynamodb::Client;
use std::time::Instant;

use crate::{ClientFactory, DbModel, PersistenceError};

use super::{TransactionGet, TransactionWrite};

pub struct TransactionExecutor {
    client: Client,
}

impl TransactionExecutor {
    pub fn new(client_factory: &ClientFactory) -> Self {
        Self {
            client: client_factory.create_client(),
        }
    }

    pub async fn send_get<T1: DbModel, T2: DbModel>(
        &self,
        transaction: TransactionGet,
    ) -> Result<(Option<T1>, Option<T2>), PersistenceError> {
        let start = Instant::now();
        let res = transaction.send(&self.client).await?;
        let duration = start.elapsed();

        tracing::info!(":{} took {:?}", stringify!(send_get), duration);

        Ok(res)
    }

    pub async fn send_write(&self, transaction: TransactionWrite) -> Result<(), PersistenceError> {
        let start = Instant::now();
        transaction.send(&self.client).await?;
        let duration = start.elapsed();

        tracing::info!(":{} took {:?}", stringify!(send_get), duration);

        Ok(())
    }
}
