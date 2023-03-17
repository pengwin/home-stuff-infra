mod dynamo;
mod inmemory;
mod persistence_layer;
mod user_cred_repository;
mod user_repository;
mod user_tx_repository;

pub use persistence_layer::PersistenceLayer;
pub use persistence_layer::PersistenceLayerFactory;
pub use user_cred_repository::UserCredDbModel;
pub use user_cred_repository::UserCredRepository;
pub use user_repository::UserDbModel;
pub use user_repository::UserRepository;
pub use user_tx_repository::UserTxRepository;

#[cfg(feature = "persistence_dynamo")]
pub use dynamo::DynamoPersistenceLayer;
#[cfg(feature = "persistence_dynamo")]
pub use dynamo::DynamoPersistenceLayerFactory;

#[cfg(feature = "persistence_inmemory")]
pub use inmemory::InmemoryPersistenceLayer;
#[cfg(feature = "persistence_inmemory")]
pub use inmemory::InmemoryPersistenceLayerFactory;
