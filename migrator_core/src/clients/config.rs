use crate::error::{ErrorType};
use crate::result::Result;
use std::str::FromStr;
use crate::clients::driver::DriverType;
use url::{Url};
use std::borrow::Cow;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref CONFIGURATION_PATH: String = std::env::var("CONFIGURATION_PATH")
        .unwrap_or_else(|_| "clickhouse-migrator.toml".to_owned());
}

const VERSION: &str = "0.1.0";

pub fn load_config() -> Result<Config> {
    match confy::load::<Config>(&CONFIGURATION_PATH.to_string()) {
        Ok(res) => Ok(res),
        Err(e) => Err(ErrorType::UnableToReadConfig(e)),
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Config {
    pub driver: DriverType,
    pub migrations: Option<String>,
    pub uri: Option<String>,
    db_host: Option<String>,
    db_user_name: Option<String>,
    db_pass: Option<String>,
    db_port: Option<i32>,
    db_database: Option<String>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            driver: DriverType::ClickHouseDriver,
            uri: Some("http://localhost:8083".to_string()),
            migrations: None,
            db_host: None,
            db_user_name: None,
            db_pass: None,
            db_port: None,
            db_database: None
        }
    }
}

impl Config {
    pub fn new(driver: &str) -> Result<Config> {
        let driver_type = DriverType::from_str(driver)?;

        Ok(Self {
            driver: driver_type,
            uri: None,
            migrations: None,
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

    pub fn build_uri(&self) -> String {
        if let Some(uri) = &self.uri {
            uri.clone()
        } else {
            let driver = self.driver.clone();

            let mut url: String = driver.prefix().to_string() + "://";

            url = url + if let Some(host) = &self.db_host { host } else { "localhost" };

            if let Some(port) = &self.db_port {
                url = url + &format!(":{}/", port.to_string().as_str());
            } else {
                url = url + &format!(":{}/", 8123.to_string().as_str());
            }

            if let Some(username) = &self.db_user_name {
                url = url + "?username=" + username;
            }

            if let Some(password) = &self.db_pass {
                url = url + "?password=" + password;
            }

            url
        }
    }

    pub fn write(&self) -> Result<()> {
        confy::store(&CONFIGURATION_PATH.to_string(), self).map_err(ErrorType::UnableToWriteConfig)
    }
}