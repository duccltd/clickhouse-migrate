use crate::table::Table;
use std::fmt::Formatter;

// TODO: Engine parameters
pub enum Engines {
    MergeTree,

    ReplicatedMergeTree(String, String),
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

impl Engines {
    pub fn as_str(&self) -> &'static str {
        match self {
            Engines::MergeTree => "MergeTree()",

            /*
            Engines::ReplicatedMergeTree(a, b) => {},
            Engines::ReplicatedSummingMergeTree => "ReplicatedSummingMergeTree",
            Engines::ReplicatedReplacingMergeTree => "ReplicatedReplacingMergeTree",
            Engines::ReplicatedAggregatingMergeTree => "ReplicatedAggregatingMergeTree",
            Engines::ReplicatedCollapsingMergeTree => "ReplicatedCollapsingMergeTree",
            Engines::ReplicatedVersionedCollapsingMergeTree => "ReplicatedVersionedCollapsingMergeTree",
            Engines::ReplicatedGraphiteMergeTree => "ReplicatedGraphiteMergeTree",

            Engines::ReplacingMergeTree => "ReplacingMergeTree",
            Engines::SummingMergeTree => "SummingMergeTree",
            Engines::AggregatingMergeTree => "AggregatingMergeTree",
            Engines::CollapsingMergeTree => "CollapsingMergeTree",
            Engines::VersionedCollapsingMergeTree => "VersionedCollapsingMergeTree",
            Engines::GraphiteMergeTree => "GraphiteMergeTree",
             */

            Engines::StripeLog => "StripeLog",
            Engines::Log => "Log",
            Engines::TinyLog => "TinyLog",

            _ => unimplemented!()
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