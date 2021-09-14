
use async_trait::async_trait;
use crate::result::Result;

#[async_trait]
pub trait Transaction {
    async fn execute_many(&mut self, queries: &[&str]) -> Result<()>;

    async fn execute_query(&mut self, query: &str) -> Result<()>;
}

#[async_trait]
pub trait RowFetcher<T> {
    async fn fetch_one(&mut self, query: &str) -> Result<T>;

    async fn fetch_many(&mut self, query: &str) -> Result<Vec<T>>;
}