pub struct PasswordHasher {
    inner: Hasher,
}

impl Display for PasswordHasher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            Hasher::Ring(r) => write!(f, "PasswordHasher: {}", r),
            Hasher::Sha2(s) => write!(f, "PasswordHasher: {}", s),
        }
    }
}

impl PasswordHasher {
    pub fn new(pepper: &str) -> Self {
        Self::new_ring(pepper)
    }

    pub fn new_sha2(pepper: &str) -> Self {
        Self {
            inner: Hasher::Sha2(Sha2PasswordHasher::new(pepper)),
        }
    }

    pub fn new_ring(pepper: &str) -> Self {
        Self {
            inner: Hasher::Ring(RingPasswordHasher::new(pepper)),
        }
    }

    pub fn hash_password(&self, password: &str, salt: &str) -> String {
        match &self.inner {
            Hasher::Ring(r) => r.hash_password(password, salt),
            Hasher::Sha2(s) => s.hash_password(password, salt),
        }
    }
}

enum Hasher {
    Ring(RingPasswordHasher),
    Sha2(Sha2PasswordHasher),
}

use std::fmt::Display;

use data_encoding::HEXLOWER;
use ring::digest;

pub struct RingPasswordHasher {
    pepper: String,
}

impl Display for RingPasswordHasher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RingPasswordHasher: {}", self.pepper)
    }
}

impl RingPasswordHasher {
    pub fn new(pepper: &str) -> Self {
        Self {
            pepper: pepper.to_owned(),
        }
    }

    pub fn hash_password(&self, password: &str, salt: &str) -> String {
        let digest = digest::digest(
            &digest::SHA512,
            str_for_hash(password, salt, &self.pepper).as_bytes(),
        );
        HEXLOWER.encode(digest.as_ref())
    }
}

use sha2::{Digest, Sha512};

pub struct Sha2PasswordHasher {
    pepper: String,
}

impl Display for Sha2PasswordHasher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sha2PasswordHasher: {}", self.pepper)
    }
}

impl Sha2PasswordHasher {
    pub fn new(pepper: &str) -> Self {
        Self {
            pepper: pepper.to_owned(),
        }
    }

    pub fn hash_password(&self, password: &str, salt: &str) -> String {
        let mut hasher = Sha512::new();
        hasher.update(str_for_hash(password, salt, &self.pepper));
        format!("{:x}", hasher.finalize())
    }
}

fn str_for_hash(password: &str, salt: &str, pepper: &str) -> String {
    format!("{}{}{}", salt, password, pepper)
}

#[cfg(test)]
mod tests {
    use super::PasswordHasher;

    #[test]
    fn compare_hasher() {
        let pepper = "pepper";
        let salt = "salt";
        let password = "password";
        let sha2 = PasswordHasher::new_sha2(pepper);
        let ring = PasswordHasher::new_ring(pepper);
        assert_eq!(
            ring.hash_password(password, salt),
            sha2.hash_password(password, salt)
        );
    }
}
