use super::error::{Error, Result};

use {
    async_trait::async_trait,
    serde::Deserialize,
    sha2::{Digest, Sha256},
};

#[async_trait]
pub trait Model: Sized {
    type Id;

    async fn find(id: Self::Id) -> sqlx::Result<Self>;
    async fn find_batch(ids: &[Self::Id]) -> sqlx::Result<Vec<Self>>;
    async fn find_all() -> sqlx::Result<Vec<Self>>;
    async fn insert(value: &Self) -> sqlx::Result<Option<Self::Id>>;
    async fn delete(id: Self::Id) -> sqlx::Result<()>;
}

#[derive(Deserialize)]
pub struct Protected<T> {
    inner: T,
    pass: String,
}

impl<T> Protected<T> {
    pub fn validate(self) -> Result<T> {
        let env_pass = dotenv::var("ADMIN_PASS").map_err(Error::from)?;

        let mut hasher = Sha256::new();
        hasher.update(env_pass.as_bytes());
        let env_pass = Vec::from_iter(hasher.finalize().into_iter());

        (self.pass.as_bytes() == env_pass)
            .then_some(self.inner)
            .ok_or(Error::Unauthorized)
    }
}
