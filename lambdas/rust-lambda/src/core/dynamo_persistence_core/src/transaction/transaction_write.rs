use std::collections::HashMap;

use aws_sdk_dynamodb::{
    model::{AttributeValue, Delete, Put, TransactWriteItem},
    Client,
};

use crate::{db_model::DbModel, PersistenceError, ToAttribute};

enum WriteItemDescriptor {
    Put {
        table: String,
        item: HashMap<String, AttributeValue>,
    },
    Delete {
        table: String,
        hash_key: String,
        id: AttributeValue,
    },
}

pub struct TransactionWriteBuilder {
    items: Vec<WriteItemDescriptor>,
}

impl TransactionWriteBuilder {
    pub fn put<T: DbModel>(mut self, dto: &T) -> Result<TransactionWriteBuilder, PersistenceError> {
        let item = serde_dynamo::to_item(dto)?;

        self.items.push(WriteItemDescriptor::Put {
            table: T::table(),
            item,
        });
        Ok(self)
    }

    pub fn delete<T: DbModel, A: ToAttribute>(mut self, id: A) -> TransactionWriteBuilder {
        self.items.push(WriteItemDescriptor::Delete {
            table: T::table(),
            hash_key: T::hash_key(),
            id: id.to_attribute(),
        });
        self
    }

    pub fn delete_by_index<T: DbModel, A: ToAttribute>(
        mut self,
        hash_key: &str,
        id: A,
    ) -> TransactionWriteBuilder {
        self.items.push(WriteItemDescriptor::Delete {
            table: T::table(),
            hash_key: hash_key.to_owned(),
            id: id.to_attribute(),
        });
        self
    }

    pub fn build(self) -> TransactionWrite {
        let items: Vec<TransactWriteItem> = self.items.into_iter().map(Self::map).collect();

        TransactionWrite { items }
    }

    fn map(d: WriteItemDescriptor) -> TransactWriteItem {
        match d {
            WriteItemDescriptor::Put { table, item } => TransactWriteItem::builder()
                .put(
                    Put::builder()
                        .table_name(table)
                        .set_item(Some(item))
                        .build(),
                )
                .build(),
            WriteItemDescriptor::Delete {
                table,
                hash_key,
                id,
            } => TransactWriteItem::builder()
                .delete(
                    Delete::builder()
                        .table_name(table)
                        .key(hash_key, id)
                        .build(),
                )
                .build(),
        }
    }
}

pub struct TransactionWrite {
    items: Vec<TransactWriteItem>,
}

impl TransactionWrite {
    pub fn builder() -> TransactionWriteBuilder {
        TransactionWriteBuilder { items: Vec::new() }
    }

    pub(super) async fn send(self, client: &Client) -> Result<(), PersistenceError> {
        let mut t = client.transact_write_items();

        for i in self.items {
            t = t.transact_items(i);
        }

        t.send().await?;

        Ok(())
    }
}
