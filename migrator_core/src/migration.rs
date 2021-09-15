use crate::table::Table;
use std::fmt::Formatter;

// TODO: Engine parameters
pub enum Engines {
    MergeTree,

    ReplicatedMergeTree,
    ReplicatedSummingMergeTree,
    ReplicatedReplacingMergeTree,
    ReplicatedAggregatingMergeTree,
    ReplicatedCollapsingMergeTree,
    ReplicatedVersionedCollapsingMergeTree,
    ReplicatedGraphiteMergeTree,

    ReplacingMergeTree,
    SummingMergeTree,
    AggregatingMergeTree,
    CollapsingMergeTree,
    VersionedCollapsingMergeTree,
    GraphiteMergeTree,

    StripeLog,
    Log,
    TinyLog
}

impl std::fmt::Display for Engines {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Engines::MergeTree => f.write_str("MergeTree"),

            Engines::ReplicatedMergeTree => f.write_str("ReplicatedMergeTree"),
            Engines::ReplicatedSummingMergeTree => f.write_str("ReplicatedSummingMergeTree"),
            Engines::ReplicatedReplacingMergeTree => f.write_str("ReplicatedReplacingMergeTree"),
            Engines::ReplicatedAggregatingMergeTree => f.write_str("ReplicatedAggregatingMergeTree"),
            Engines::ReplicatedCollapsingMergeTree => f.write_str("ReplicatedCollapsingMergeTree"),
            Engines::ReplicatedVersionedCollapsingMergeTree => f.write_str("ReplicatedVersionedCollapsingMergeTree"),
            Engines::ReplicatedGraphiteMergeTree => f.write_str("ReplicatedGraphiteMergeTree"),

            Engines::ReplacingMergeTree => f.write_str("ReplacingMergeTree"),
            Engines::SummingMergeTree => f.write_str("SummingMergeTree"),
            Engines::AggregatingMergeTree => f.write_str("AggregatingMergeTree"),
            Engines::CollapsingMergeTree => f.write_str("CollapsingMergeTree"),
            Engines::VersionedCollapsingMergeTree => f.write_str("VersionedCollapsingMergeTree"),
            Engines::GraphiteMergeTree => f.write_str("GraphiteMergeTree"),

            Engines::StripeLog => f.write_str("StripeLog"),
            Engines::Log => f.write_str("Log"),
            Engines::TinyLog => f.write_str("TinyLog")
        }
    }
}

pub struct Migration {
    engine: Engines,
}

impl Migration {
    pub fn new(engine: Engines) -> Migration {
        Migration {
            engine,
        }
    }

    pub fn default() -> Migration {
        Migration {
            engine: Engines::MergeTree
        }
    }

    pub fn create_table<F: 'static>(self, table_name: &str, column_gen: F) -> Table
    where
        F: Fn(&mut Table)
    {
        let mut table = Table::new(table_name.to_string());

        column_gen(&table);

        table
    }

    pub fn create_table_if_not_exists<F: 'static>(self, table_name: &str, column_gen: F) -> Table
    where
        F: Fn(&mut Table)
    {
        let mut table = Table::new_exists(table_name.to_string(), true);

        column_gen(&table);

        table
    }

    pub fn exec(self) {
        unimplemented!("execution")
    }
}