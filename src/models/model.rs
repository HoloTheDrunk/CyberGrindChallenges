#[async_trait::async_trait]
pub trait Model: Sized {
    type Id;

    async fn find(id: Self::Id) -> sqlx::Result<Self>;
    async fn find_batch(ids: &[Self::Id]) -> sqlx::Result<Vec<Self>>;
    async fn find_all() -> sqlx::Result<Vec<Self>>;
    async fn insert(value: &Self) -> sqlx::Result<Option<Self::Id>>;
    async fn delete(id: Self::Id) -> sqlx::Result<()>;
}
