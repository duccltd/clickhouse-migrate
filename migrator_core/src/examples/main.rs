use crate::migration::{Migration, Engines};
use crate::table::Types;

fn main() {
    let mut m = Migration::new(Engines::MergeTree);

    m.create_table_if_not_exists("events", |t| {
        t.column("placement_id", Types::Varchar(255)).primary();
    });

    m.exec();
}