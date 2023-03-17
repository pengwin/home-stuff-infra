use dynamo_persistence_core::PersistenceError;

use crate::handlers::users;

pub async fn create(
    test_login: &str,
    test_password: &str,
    pepper: &str,
    persistence_layer: &super::InmemoryPersistenceLayer,
) -> Result<(), PersistenceError> {
    use crate::{persistence::PersistenceLayer, security::PasswordHasher};

    let password_hasher = PasswordHasher::new(pepper);

    let (user, user_cred) = users::create_user(
        "test_run_admin",
        test_login,
        test_password,
        true,
        None,
        &password_hasher,
    );

    let user_tx_repository = persistence_layer.user_tx_repository();

    user_tx_repository.save(&user, &user_cred).await
}
