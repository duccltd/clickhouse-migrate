pub mod clickhouse;
pub mod config;
pub mod driver;
pub mod traits;

pub const CREATE_CLICKHOUSE_MIGRATIONS_TABLE_QUERY: &str = "
CREATE TABLE IF NOT EXISTS clickhouse_migrations (
    timestamp DateTime64(9) NOT NULL,
    name TEXT NOT NULL,
    checksum TEXT NOT NULL
)
engine=TinyLog
";

pub const CREATE_CLICKHOUSE_LOCK_TABLE_QUERY: &str = "
CREATE TABLE IF NOT EXISTS clickhouse_migration_lock (
    is_locked UInt8 NOT NULL
)
engine=Memory
";
