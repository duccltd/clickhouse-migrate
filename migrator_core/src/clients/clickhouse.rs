use crate::clients::traits::{RowFetcher, Transaction};
use crate::result::Result;
use async_trait::async_trait;
use clickhouse::{Client as ClickHouse, Row as ClickhouseRow};
use serde::{Deserialize, Serialize};

pub trait DatabaseClient: Transaction + RowFetcher<MigrationsRow> + RowFetcher<LockRow> {}

impl<T: Transaction + RowFetcher<MigrationsRow> + RowFetcher<LockRow>> DatabaseClient for T {}

#[derive(Debug, ClickhouseRow, Deserialize, Serialize)]
pub struct MigrationsRow {
    pub timestamp: u64,
    pub name: String,
    pub checksum: String,
}

#[derive(Debug, ClickhouseRow, Deserialize, Serialize)]
pub struct LockRow {
    pub is_locked: u8,
}

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

#[async_trait]
impl RowFetcher<LockRow> for ClickHouse {
    async fn fetch_one(&mut self, query: &str) -> Result<LockRow> {
        let row = self.query(query).fetch_one::<LockRow>().await?;

        Ok(row)
    }

    async fn fetch_many(&mut self, query: &str) -> Result<Vec<LockRow>> {
        let rows = self.query(query).fetch_all::<LockRow>().await?;

        Ok(rows)
    }
}

#[async_trait]
impl RowFetcher<MigrationsRow> for ClickHouse {
    async fn fetch_one(&mut self, query: &str) -> Result<MigrationsRow> {
        let row = self.query(query).fetch_one::<MigrationsRow>().await?;

        Ok(row)
    }

    async fn fetch_many(&mut self, query: &str) -> Result<Vec<MigrationsRow>> {
        let rows = self.query(query).fetch_all::<MigrationsRow>().await?;

        Ok(rows)
    }
}
