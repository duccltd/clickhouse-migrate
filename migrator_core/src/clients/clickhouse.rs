use crate::clients::traits::{Transaction};
use clickhouse::{
    Client as ClickHouse
};
use async_trait::async_trait;
use crate::result::Result;
use crate::error::ErrorType;

#[async_trait]
impl Transaction for ClickHouse {
    async fn execute_many(&mut self, queries: &[&str]) -> Result<()> {
        for query in queries {
            self.query(query).execute().await?;
        }
        Ok(())
    }

    async fn execute_query(&mut self, query: &str) -> Result<()> {
        self.query(query).execute().await?;
        Ok(())
    }

    async fn fetch_one<T>(&mut self, query: &str) -> Result<T>
        where Self: Sized
    {
        match self.query(query)
            .fetch_one::<T>()
            .await? {
            Ok(row) => Ok(row),
            Err(e) => Err(ErrorType::RowNotFound(e))
        }
    }

    async fn fetch_many<T>(&mut self, query: &str) -> Result<Vec<T>>
        where Self: Sized
    {
        let rows = self.query(query)
            .fetch_all::<T>()
            .await?;

        Ok(rows)
    }
}