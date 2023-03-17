use serde::{de::DeserializeOwned, Serialize};

pub trait DbModel: Serialize + DeserializeOwned {
    fn table() -> String;
    fn hash_key() -> String;
}
