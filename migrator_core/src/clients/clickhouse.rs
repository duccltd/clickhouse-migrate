use crate::clients::traits::{Transaction};
use clickhouse::{
    Client as ClickHouse
};
use async_trait::async_trait;
use crate::result::Result;

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
}