use std::time::Instant;
use std::{fmt::Display, marker::PhantomData};

use aws_sdk_dynamodb::{model::AttributeValue, Client};

use crate::ClientFactory;

use super::db_model::DbModel;

pub struct Repository<T: DbModel> {
    table: String,
    client: Client,
    t: PhantomData<T>,
}

impl<T: DbModel> Repository<T> {
    pub fn new(client_factory: &ClientFactory) -> Self {
        Self {
            table: T::table(),
            client: client_factory.create_client(),
            t: PhantomData::default(),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<T>, super::error::PersistenceError> {
        let start = Instant::now();
        let result = self.client.scan().table_name(&self.table).send().await?;
        let duration = start.elapsed();

        tracing::info!(":{} took {:?}", stringify!(get_all), duration);

        if let Some(items) = result.items {
            let items: Vec<T> = serde_dynamo::from_items(items)?;

            return Ok(items);
        }

        Ok(vec![])
    }

    pub async fn query_by_index<K: Display>(
        &self,
        index_name: &str,
        hash_key: &str,
        id: K,
    ) -> Result<Vec<T>, super::error::PersistenceError> {
        let start = Instant::now();
        let result = self
            .client
            .query()
            .table_name(&self.table)
            .index_name(index_name)
            .key_condition_expression(format!("{} = :hashKey", hash_key))
            .expression_attribute_values(":hashKey", AttributeValue::S(id.to_string()))
            .send()
            .await?;
        let duration = start.elapsed();

        tracing::info!(":{} took {:?}", stringify!(query_by_index), duration);

        if let Some(items) = result.items {
            let items: Vec<T> = serde_dynamo::from_items(items)?;

            return Ok(items);
        }

        Ok(vec![])
    }

    pub async fn add(&self, item: &T) -> Result<(), super::error::PersistenceError> {
        let item = serde_dynamo::to_item(item)?;

        let start = Instant::now();
        self.client
            .put_item()
            .table_name(&self.table)
            .set_item(Some(item))
            .send()
            .await?;

        let duration = start.elapsed();

        tracing::info!(":{} took {:?}", stringify!(add), duration);

        Ok(())
    }

    pub async fn get<K: Display>(
        &self,
        id: K,
    ) -> Result<Option<T>, super::error::PersistenceError> {
        let start = Instant::now();
        let response = self
            .client
            .get_item()
            .table_name(&self.table)
            .key(T::hash_key(), AttributeValue::S(id.to_string()))
            .send()
            .await?;

        let duration = start.elapsed();

        tracing::info!(":{} took {:?}", stringify!(add), duration);

        match response.item() {
            Some(item) => {
                let result = serde_dynamo::from_item(item.clone())?;

                Ok(result)
            }
            None => Ok(None),
        }
    }

    pub async fn delete<K: Display>(&self, id: K) -> Result<(), super::error::PersistenceError> {
        let start = Instant::now();
        self.client
            .delete_item()
            .table_name(&self.table)
            .key(T::hash_key(), AttributeValue::S(id.to_string()))
            .send()
            .await?;

        let duration = start.elapsed();

        tracing::info!(":{} took {:?}", stringify!(add), duration);

        Ok(())
    }
}
