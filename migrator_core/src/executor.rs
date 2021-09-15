use crate::util;
use crate::reader;
use crate::archive::LocalVersionArchive;
use crate::clients::config::Config;
use crate::clients::driver::Driver;
use tracing::*;
use crate::clients::clickhouse::DatabaseClient;

pub async fn run(connection: Box<dyn DatabaseClient>, path: &str) {
    let location = util::standardise_path(path);

    let migrations = reader::find_migration_files(
        location.clone()
    ).expect("no migrations found");

    let archive = LocalVersionArchive::from(location);

    let mut driver = Driver::new(connection);

    let report = driver
        .migrate(migrations, archive)
        .await
        .expect("could not generate report");

    info!("{}", report);
}