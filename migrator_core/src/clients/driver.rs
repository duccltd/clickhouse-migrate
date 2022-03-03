use crate::error::ErrorType;
use crate::clients::traits::{Transaction, RowFetcher};
use crate::clients::config::Config;
use crate::dbl::{MigrationFile};
use clickhouse::{Client as ClickHouse};
use crate::clients::CREATE_CLICKHOUSE_LOCK_TABLE_QUERY;
use crate::result::Result;
use tracing::*;
use chrono::{DateTime, Local};
use crate::clients::clickhouse::{MigrationLockRow, DatabaseClient};
use crate::report::ExecutionReport;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub enum DriverType {
    ClickHouseDriver
}

impl DriverType {
    pub fn prefix(&self) -> &'static str {
        match *self {
            DriverType::ClickHouseDriver => "tcp"
        }
    }
}

impl std::str::FromStr for DriverType {
    type Err = ErrorType;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let driver = match s {
            "clickhouse" => DriverType::ClickHouseDriver,
            _ => return Err(ErrorType::InvalidDriverType(s.to_string()))
        };
        Ok(driver)
    }
}

pub struct Driver {
    client: Box<dyn DatabaseClient>
}

impl Driver {
    pub fn new(client: Box<dyn DatabaseClient>) -> Driver {
        Driver {
            client,
        }
    }

    pub fn from_config(config: Config) -> Driver {
        let driver_type = config.driver.clone();
        let uri = config.build_uri();

        let client = match driver_type {
            DriverType::ClickHouseDriver => Box::new(ClickHouse::default().with_url(uri))
        };

        Driver {
            client,
        }
    }

    pub async fn run_migrations(&mut self) -> Result<Vec<MigrationLockRow>> {
        let entries = self.client.fetch_many("SELECT * FROM migration_lock").await?;

        Ok(entries)
    }

    pub async fn migrate(&mut self, migrations: Vec<MigrationFile>) -> Result<ExecutionReport> {
        let mut ran_migrations = Vec::new();

        self.client.execute_query(CREATE_CLICKHOUSE_LOCK_TABLE_QUERY).await?;

        let run_migrations = self.run_migrations().await?;

        for mut migration in migrations {
            let existing = &run_migrations.iter().find(|m| m.name == migration.name);

            // Skip if this has already been run
            if let Some(existing) = existing {
                // Panic if the migration directory is corrupt!
                if migration.checksum().to_string() != existing.checksum {
                    panic!("{} != {}. Migration directory is corrupt.", migration.checksum(), existing.checksum);
                }
                continue;
            }

            self.client.execute_many(&[migration.sql(), &migration.to_insert_sql()]).await?;

            ran_migrations.push(migration.clone());

            debug!("Ran migration {}", migration.name())
        }

        Ok(ExecutionReport::new(ran_migrations))
    }
}