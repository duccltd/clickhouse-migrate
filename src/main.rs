use migration_initiator::{reader, util, cli};
use migration_initiator::clients::config::{Config, Driver};
use anyhow::Result;
use migration_initiator::archive::Archive;
use tracing::*;

// cargo run -- --driver clickhouse --url http://localhost:8123 --migrations ../clickhouse_migrations
#[tokio::main]
async fn main() -> Result<()> {
    let opts: cli::Opts = cli::parse();

    let location = util::standardise_path(&opts.migrations.to_string());

    let migrations = reader::find_migration_files(location.clone())
        .expect("no migrations found");

    let archive = Archive::from(location);

    if let Ok(config) = Config::new(&opts.driver) {
        let config = config.uri(&opts.url);

        let mut driver = Driver::new(config);

        let report = driver
            .migrate(migrations, archive)
            .await
            .expect("could not generate report");

        info!("{}", report);
    } else {
        error!("could not generate config");
    }

    Ok(())
}
