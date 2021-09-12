use crate::error::{ErrorType};
use clickhouse::{Client as ClickHouse};
use crate::result::Result;
use std::str::FromStr;
use crate::clients::traits::Transaction;

use crate::migrator::{Migration, ExecutionReport};

use crate::clients::CREATE_CLICKHOUSE_LOCK_TABLE_QUERY;
use crate::archive::Archive;
use tracing::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum DriverType {
    ClickHouseDriver
}

impl DriverType {
    fn prefix(&self) -> &'static str {
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
    client: Box<dyn Transaction>
}

impl Driver {
    pub fn new(config: Config) -> Driver {
        let driver_type = config.driver.clone();
        let uri = build_uri(&config);

        let client = match driver_type {
            DriverType::ClickHouseDriver => Box::new(ClickHouse::default().with_url(uri))
        };

        Driver {
            client,
        }
    }

    pub async fn get_last_version(&self, _migration: &str) -> Result<()> {
        Ok(())
    }

    pub async fn migrate(&mut self, migrations: Vec<Migration>, mut archive: Archive) -> Result<ExecutionReport> {
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

            migration.set_executed_at();

            self.client.execute_many(&[migration.sql(), &migration.to_insert_sql()]).await?;

            ran_migrations.push(migration.clone());

            archive.add_migration_version(migration.clone());

            debug!("Ran migration {}", migration.name())
        }

        Ok(ExecutionReport::new(ran_migrations))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    driver: DriverType,
    uri: Option<String>,
    db_host: Option<String>,
    db_user_name: Option<String>,
    db_pass: Option<String>,
    db_port: Option<i32>,
    db_database: Option<String>,
}

impl Config {
    pub fn new(driver: &str) -> Result<Config> {
        let driver_type = DriverType::from_str(driver)?;

        Ok(Config {
            driver: driver_type,
            uri: None,
            db_host: None,
            db_user_name: None,
            db_pass: None,
            db_port: None,
            db_database: None
        })
    }

    pub fn uri(self, uri: &str) -> Config {
        Self {
            uri: Some(uri.into()),
            ..self
        }
    }

    pub fn host(self, host: &str) -> Config {
        Self {
            db_host: Some(host.into()),
            ..self
        }
    }

    pub fn user_name(self, user_name: &str) -> Config {
        Self {
            db_user_name: Some(user_name.into()),
            ..self
        }
    }

    pub fn db_pass(self, db_pass: &str) -> Config {
        Self {
            db_pass: Some(db_pass.into()),
            ..self
        }
    }

    pub fn db_port(self, db_port: &i32) -> Config {
        Self {
            db_port: Some(*db_port),
            ..self
        }
    }

    pub fn db_database(self, db_database: &str) -> Config {
        Self {
            db_database: Some(db_database.into()),
            ..self
        }
    }
}

pub fn build_uri(config: &Config) -> String {
    if let Some(uri) = &config.uri {
        uri.clone()
    } else {
        let driver = config.driver.clone();

        let mut url: String = driver.prefix().to_string() + "://";

        url = url + if let Some(host) = &config.db_host { host } else { "localhost" };

        if let Some(port) = &config.db_port {
            url = url + &format!(":{}/", port.to_string().as_str());
        } else {
            url = url + &format!(":{}/", 8123.to_string().as_str());
        }

        if let Some(username) = &config.db_user_name {
            url = url + "?username=" + username;
        }

        if let Some(password) = &config.db_pass {
            url = url + "?password=" + password;
        }

        url
    }
}