use crate::clients::traits::{Transaction, RowFetcher};
use clickhouse::{Client as ClickHouse, Row as ClickhouseRow};
use async_trait::async_trait;
use crate::result::Result;
use crate::error::ErrorType;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

pub trait DatabaseClient: Transaction + RowFetcher<MigrationLockRow> {}

impl <T: Transaction + RowFetcher<MigrationLockRow>> DatabaseClient for T {}

#[derive(Debug, ClickhouseRow, Deserialize, Serialize)]
pub struct MigrationLockRow {
    pub timestamp: u64,
    pub name: String,
    pub checksum: String
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
impl RowFetcher<MigrationLockRow> for ClickHouse {
    async fn fetch_one(&mut self, query: &str) -> Result<MigrationLockRow> {
        let row = self.query(query)
            .fetch_one::<MigrationLockRow>()
            .await?;

        Ok(row)
    }

    async fn fetch_many(&mut self, query: &str) -> Result<Vec<MigrationLockRow>> {
        let rows = self.query(query)
            .fetch_all::<MigrationLockRow>()
            .await?;

        Ok(rows)
    }
}