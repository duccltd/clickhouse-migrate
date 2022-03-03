use crate::error::ErrorType;
use crate::clients::traits::{Transaction, RowFetcher};
use crate::clients::config::Config;
use crate::dbl::{MigrationFile};
use clickhouse::{Client as ClickHouse};
use crate::clients::CREATE_CLICKHOUSE_LOCK_TABLE_QUERY;
use crate::archive::LocalVersionArchive;
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

    pub async fn last_version(&mut self, migration_name: &str) -> Result<Option<MigrationLockRow>> {
        let entry = self.client.fetch_one(
            &format!(
                "SELECT * FROM migration_lock WHERE name = {} LIMIT 1",
                migration_name
            )
        ).await?;

        Ok(Some(entry))
    }

    pub async fn migrate(&mut self, migrations: Vec<MigrationFile>, mut archive: LocalVersionArchive) -> Result<ExecutionReport> {
        let mut ran_migrations = Vec::new();

        self.client.execute_query(CREATE_CLICKHOUSE_LOCK_TABLE_QUERY).await?;

        for mut migration in migrations {
            let latest_migration = self.last_version(migration.name()).await?;

            if let Some(latest_migration) = latest_migration {
                if migration.checksum() == &latest_migration.checksum {
                    debug!("{} == {}", migration.checksum(), latest_migration.checksum);
                    continue;
                }

                migration.set_version(latest_migration.next_version());
            }

            self.client.execute_many(&[migration.sql(), &migration.to_insert_sql()]).await?;

            ran_migrations.push(migration.clone());

            archive.add_migration_version(migration.clone());

            debug!("Ran migration {}", migration.name())
        }

        Ok(ExecutionReport::new(ran_migrations))
    }
}