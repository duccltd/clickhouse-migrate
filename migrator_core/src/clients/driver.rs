use crate::error::ErrorType;
use crate::clients::traits::Transaction;
use crate::clients::config::Config;
use crate::migration::{Migration, ExecutionReport};
use clickhouse::{Client as ClickHouse};
use crate::clients::CREATE_CLICKHOUSE_LOCK_TABLE_QUERY;
use crate::archive::LocalVersionArchive;
use crate::result::Result;
use tracing::*;
use chrono::{DateTime, Local};

#[derive(Clone, Eq, PartialEq, Debug)]
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

struct MigrationLockRow {
    timestamp: DateTime<Local>,
    name: String,
    version: i32,
    checksum: i64
}

pub struct Driver {
    client: Box<dyn Transaction>
}

impl Driver {
    pub fn new(config: Config) -> Driver {
        let driver_type = config.driver.clone();
        let uri = config.build_uri();

        let client = match driver_type {
            DriverType::ClickHouseDriver => Box::new(ClickHouse::default().with_url(uri))
        };

        Driver {
            client,
        }
    }

    pub async fn last_version(&mut self, migration: Migration) -> Result<Option<MigrationLockRow>> {
        let entry = match self.client.fetch_one::<MigrationLockRow>(
            &format!(
                "SELECT * FROM migration_lock WHERE name = {} LIMIT 1",
                migration.name()
            )
        ).await {
            Ok(entry) => Some(entry),
            Err(_e) => None
        };

        Ok(entry)
    }

    pub async fn migrate(&mut self, migrations: Vec<Migration>, mut archive: LocalVersionArchive) -> Result<ExecutionReport> {
        let mut ran_migrations = Vec::new();

        self.client.execute_query(CREATE_CLICKHOUSE_LOCK_TABLE_QUERY).await?;

        for mut migration in migrations {
            let latest_migration = archive.get_latest_version(migration.name());

            if let Some(latest_migration) = latest_migration {
                if migration.checksum() == latest_migration.checksum() {
                    debug!("{} == {}", migration.checksum(), latest_migration.checksum());
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