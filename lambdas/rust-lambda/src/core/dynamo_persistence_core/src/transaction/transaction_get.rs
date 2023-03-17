use aws_sdk_dynamodb::{
    model::{AttributeValue, Get, ItemResponse, TransactGetItem},
    output::TransactGetItemsOutput,
    Client,
};

use crate::{db_model::DbModel, PersistenceError, ToAttribute};

struct GetItemDescriptor {
    table: String,
    hash_key: String,
    id: AttributeValue,
}

pub struct TransactionGetBuilder {
    items: Vec<GetItemDescriptor>,
}

impl TransactionGetBuilder {
    pub fn get<T: DbModel, A: ToAttribute>(mut self, id: A) -> TransactionGetBuilder {
        self.items.push(GetItemDescriptor {
            table: T::table(),
            hash_key: T::hash_key(),
            id: id.to_attribute(),
        });
        self
    }

    pub fn get_by_index<T: DbModel, A: ToAttribute>(
        mut self,
        hash_key: &str,
        id: A,
    ) -> TransactionGetBuilder {
        self.items.push(GetItemDescriptor {
            table: T::table(),
            hash_key: hash_key.to_owned(),
            id: id.to_attribute(),
        });
        self
    }

    pub fn build(self) -> TransactionGet {
        let items: Vec<TransactGetItem> = self
            .items
            .into_iter()
            .map(|i| {
                TransactGetItem::builder()
                    .get(
                        Get::builder()
                            .table_name(i.table)
                            .key(i.hash_key, i.id)
                            .build(),
                    )
                    .build()
            })
            .collect();

        TransactionGet { items }
    }
}

pub struct TransactionGet {
    items: Vec<TransactGetItem>,
}

impl TransactionGet {
    pub fn builder() -> TransactionGetBuilder {
        TransactionGetBuilder { items: Vec::new() }
    }

    pub(super) async fn send<T1: DbModel, T2: DbModel>(
        self,
        client: &Client,
    ) -> Result<(Option<T1>, Option<T2>), PersistenceError> {
        let mut t = client.transact_get_items();

        for i in self.items {
            t = t.transact_items(i);
        }

        let r = t.send().await?;

        Self::parse_output(r)
    }

    fn parse_output<T1: DbModel, T2: DbModel>(
        output: TransactGetItemsOutput,
    ) -> Result<(Option<T1>, Option<T2>), PersistenceError> {
        match output.responses() {
            Some(s) => {
                let len = s.len();
                let expected = 2;
                if len < 2 {
                    return Err(PersistenceError::TransactGetOutputParsingErrorError(
                        format!(
                            "Not enough elements in response {} expected {}",
                            len, expected
                        ),
                    ));
                }

                let t1 = Self::parse_item::<T1>(&s[0])?;
                let t2 = Self::parse_item::<T2>(&s[1])?;

                Ok((t1, t2))
            }
            None => Ok((None, None)),
        }
    }

    fn parse_item<T: DbModel>(item: &ItemResponse) -> Result<Option<T>, PersistenceError> {
        match item.item() {
            Some(s) => {
                let result = serde_dynamo::from_item(s.clone())?;
                Ok(result)
            }
            None => Ok(None),
        }
    }
}
