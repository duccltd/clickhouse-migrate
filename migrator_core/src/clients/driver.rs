use crate::error::ErrorType;
use crate::clients::config::Config;
use crate::migration::{MigrationFile};
use clickhouse::{Client as ClickHouse};
use crate::clients::CREATE_CLICKHOUSE_LOCK_TABLE_QUERY;
use crate::result::Result;
use tracing::*;
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

        let mut new_migrations: Vec<&MigrationFile> = vec![];

        for migration in &migrations {
            let old_migration = &run_migrations.iter().find(|m| m.name == migration.name);

            // Skip if this has already been run
            if let Some(old_migration) = old_migration {
                // Panic if the migration directory is corrupt!
                if migration.checksum().to_string() != old_migration.checksum {
                    panic!("Checksum {} != {}. Migration directory is corrupt. {:?}", migration.checksum(), old_migration.checksum, vec![&migration.name, &old_migration.name]);
                }

                continue;
            }

            // Check if valid file
            if &migration.sql == "" {
                panic!("{}. Empty migration file.", &migration.name);
            }

            new_migrations.push(migration);
        }

        // Check if any migrations are missing
        if (migrations.len() - new_migrations.len()) != run_migrations.len() {
            let missing_migrations: Vec<&String> = run_migrations.iter().filter(|rm| migrations.iter().find(|om| om.name == rm.name).is_none()).map(|em| &em.name).collect();

            panic!("Migration directory is corrupt. Missing following files: {:?}", missing_migrations);
        }

        // Create the new ones
        for migration in new_migrations {
            self.client.execute_many(&[&migration.sql, &migration.to_insert_sql()]).await?;

            ran_migrations.push(migration.clone());

            debug!("Ran migration {}", &migration.name)
        }

        Ok(ExecutionReport::new(ran_migrations))
    }
}