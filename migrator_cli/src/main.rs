mod cli;

use anyhow::Result;
use tracing::*;
use migrator_core::{util, reader};
use migrator_core::clients::config::{Config};
use migrator_core::clients::driver::Driver;
use migrator_core::archive::LocalVersionArchive;

// cargo run -- --driver clickhouse --url http://localhost:8123 --migrations ../clickhouse_migrations
#[tokio::main]
async fn main() -> Result<()> {
    let opts: cli::Opts = cli::parse();

    match opts {
        cli::Opts::Migrate { driver, url, migrations } => {
            let location = util::standardise_path(&migrations.to_string());

            let migrations = reader::find_migration_files(location.clone())
                .expect("no migrations found");

            let archive = LocalVersionArchive::from(location);

            if let Ok(config) = Config::new(&driver) {
                let config = config.uri(&url);

                let mut driver = Driver::new(config);

                let report = driver
                    .migrate(migrations, archive)
                    .await
                    .expect("could not generate report");

                info!("{}", report);
            } else {
                error!("could not generate config");
            }
        }
    }

    Ok(())
}
