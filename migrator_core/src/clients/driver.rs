use crate::clients::clickhouse::{DatabaseClient, LockRow, MigrationsRow};
use crate::clients::config::Config;
use crate::clients::{
    CREATE_CLICKHOUSE_LOCK_TABLE_QUERY, CREATE_CLICKHOUSE_MIGRATIONS_TABLE_QUERY,
};
use crate::error::ErrorType;
use crate::migration::MigrationFile;
use crate::report::ExecutionReport;
use crate::result::Result;
use clickhouse::Client as ClickHouse;
use serde::{Deserialize, Serialize};
use tracing::*;

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub enum DriverType {
    ClickHouseDriver,
}

impl DriverType {
    pub fn prefix(&self) -> &'static str {
        match *self {
            DriverType::ClickHouseDriver => "tcp",
        }
    }
}

impl std::str::FromStr for DriverType {
    type Err = ErrorType;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let driver = match s {
            "clickhouse" => DriverType::ClickHouseDriver,
            _ => return Err(ErrorType::InvalidDriverType(s.to_string())),
        };
        Ok(driver)
    }
}

pub struct Driver {
    client: Box<dyn DatabaseClient>,
}

impl Driver {
    pub fn new(client: Box<dyn DatabaseClient>) -> Driver {
        Driver { client }
    }

    pub fn from_config(config: Config) -> Driver {
        let driver_type = config.driver.clone();
        let uri = config.build_uri();

        let client = match driver_type {
            DriverType::ClickHouseDriver => Box::new(ClickHouse::default().with_url(uri)),
        };

        Driver { client }
    }

    pub async fn run_migrations(&mut self) -> Result<Vec<MigrationsRow>> {
        let entries = self
            .client
            .fetch_many("SELECT * FROM clickhouse_migrations")
            .await?;

        Ok(entries)
    }

    pub async fn lock_status(&mut self) -> Option<LockRow> {
        match self
            .client
            .fetch_one("SELECT * FROM clickhouse_migration_lock LIMIT 1")
            .await
        {
            Ok(row) => Some(row),
            Err(_e) => None,
        }
    }

    pub async fn change_lock(&mut self, status: u8) -> Result<()> {
        self.client.execute_query(
            &format!("ALTER TABLE clickhouse_migration_lock UPDATE is_locked = {} WHERE is_locked IS NOT NULL", status)
        ).await
    }

    pub async fn prerequisite(&mut self) -> Result<()> {
        let mut queries = vec![
            CREATE_CLICKHOUSE_MIGRATIONS_TABLE_QUERY,
            CREATE_CLICKHOUSE_LOCK_TABLE_QUERY,
        ];

        let lock_status = self.lock_status().await;
        if lock_status.is_none() {
            queries.push("INSERT INTO clickhouse_migration_lock (*) VALUES (0)")
        }

        self.client.execute_many(&queries).await
    }

    pub async fn rollback(&mut self, migrations: Vec<MigrationFile>) -> Result<ExecutionReport> {
        let mut run_migrations = self.run_migrations().await?;

        // Sort by which was run last
        run_migrations.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        println!("{:?}", run_migrations);

        unimplemented!()
    }

    pub async fn migrate(&mut self, migrations: Vec<MigrationFile>) -> Result<ExecutionReport> {
        // Run the prerequisite functions such as creating tables etc.
        self.prerequisite().await?;

        let mut ran_migrations = Vec::new();

        let run_migrations = self.run_migrations().await?;

        let mut new_migrations: Vec<&MigrationFile> = vec![];

        let runnable_migrations: Vec<&MigrationFile> = migrations.iter().filter(|m| !m.rollback).collect();
        for migration in &runnable_migrations {
            let old_migration = &run_migrations.iter().find(|m| m.name == migration.name);

            // Skip if this has already been run
            if let Some(old_migration) = old_migration {
                // Panic if the migration directory is corrupt!
                if migration.checksum().to_string() != old_migration.checksum {
                    panic!(
                        "Checksum {} != {}. Migration directory is corrupt. {:?}",
                        migration.checksum(),
                        old_migration.checksum,
                        vec![&migration.name, &old_migration.name]
                    );
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
        if (runnable_migrations.len() - new_migrations.len()) != run_migrations.len() {
            let missing_migrations: Vec<&String> = run_migrations
                .iter()
                .filter(|rm| runnable_migrations.iter().find(|om| om.name == rm.name).is_none())
                .map(|em| &em.name)
                .collect();

            panic!(
                "Migration directory is corrupt. Missing following files: {:?}",
                missing_migrations
            );
        }

        let lock_status = self.lock_status().await;
        if let Some(status) = lock_status {
            if status.is_locked == 1 {
                panic!("Database is currently locked, cannot run migrations");
            }
        }

        self.change_lock(1).await?;

        // Create the new ones
        for migration in new_migrations {
            self.client
                .execute_many(&[&migration.sql, &migration.to_insert_sql()])
                .await?;

            ran_migrations.push(migration.clone());

            debug!("Ran migration {}", &migration.name)
        }

        self.change_lock(0).await?;

        Ok(ExecutionReport::new(ran_migrations))
    }
}
