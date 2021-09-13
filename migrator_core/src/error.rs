use std::fmt::Formatter;
use std::sync::Arc;

#[derive(Debug)]
pub enum ErrorType {
    InvalidMigrationPath(std::io::Error),
    InvalidDriver,
    InvalidDriverType(String),
    FailedToReadMigration(std::io::Error, String),
    FailedToWriteMigration(std::io::Error, String),
    FailedToExecuteMigration(std::io::Error),
    VersionCacheInvalidType(String),
    NonExistentMigrationVersions(String),
    InvalidMigrationName(String),
    InvalidParameter,
    Clickhouse(Arc<clickhouse::error::Error>)
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            ErrorType::InvalidMigrationPath(ref e) => write!(f, "{}", e),
            ErrorType::NonExistentMigrationVersions(ref migration) => write!(f, "{}", migration),
            ErrorType::Clickhouse(ref e) => write!(f, "Clickhouse error: {}", e),
            ErrorType::InvalidDriverType(ref d) => write!(f, "invalid driver type {}", d),
            ErrorType::InvalidDriver => write!(f, "invalid driver provided"),
            ErrorType::FailedToReadMigration(ref e, ref m) => write!(f, "failed to read migration {} - {}", m, e),
            ErrorType::FailedToWriteMigration(ref e, ref m) => write!(f, "failed to write migration {} - {}", m, e),
            ErrorType::FailedToExecuteMigration(ref m) => write!(f, "failed to execute migration {}", m),
            ErrorType::VersionCacheInvalidType(ref v) => write!(f, "invalid version {}", v),
            ErrorType::InvalidMigrationName(ref n) => write!(f, "invalid name: {}, must be x.sql or v1_x.sql", n),
            _ => write!(f, "An unexpected error has occurred"),
        }
    }
}

impl From<std::io::Error> for ErrorType {
    fn from(_: std::io::Error) -> Self {
        todo!()
    }
}

impl From<clickhouse::error::Error> for ErrorType {
    fn from(e: clickhouse::error::Error) -> Self {
        ErrorType::Clickhouse(Arc::new(e))
    }
}