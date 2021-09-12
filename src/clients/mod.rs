

pub mod clickhouse;
pub mod traits;
pub mod config;

pub(crate) const CREATE_CLICKHOUSE_LOCK_TABLE_QUERY: &str = "
CREATE TABLE IF NOT EXISTS migration_lock (
    timestamp DateTime64(9) NOT NULL,
    name TEXT NOT NULL,
    version Float32 NOT NULL,
    checksum Float64 NOT NULL
)
engine=TinyLog
";