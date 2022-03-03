mod cli;

use tracing::*;
use migrator_core::{util, reader, error::ErrorType, result::Result, dbl::MigrationFile};
use migrator_core::clients::config;
use migrator_core::clients::driver::Driver;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut config = config::load_config().expect("unable to load config");

    let opts: cli::Opts = cli::parse();

    match opts {
        cli::Opts::Setup(params) => match params {
            cli::Setup::Init => {
                info!("Config file has been created.");
            }
            cli::Setup::Set(set) => {
                let mut changed = false;

                if let Some(uri) = set.uri {
                    config.uri = Some(uri);
                    changed = true;
                }

                if let Some(migrations) = set.migrations {
                    config.migrations = Some(migrations);
                    changed = true;
                }

                if changed {
                    match config.write() {
                        Ok(()) => info!("Config file has been changed"),
                        Err(e) => panic!("writing config file: {}", e),
                    }
                } else {
                    info!("Options are cluster-file, proto-file and mapping-file")
                }
            }
            cli::Setup::View => {
                info!("{:?}", config);
            }
        }
        cli::Opts::Migrate(params) => match params {
            cli::Migrate::Make(params) => {
                let migrations = match config.migrations {
                    Some(migrations) => migrations,
                    None => return Err(ErrorType::MissingConfigDefinition("Missing migrations definition".into()))
                };

                MigrationFile::create(migrations, params.name).expect("unable to write migration file")
            }
            cli::Migrate::Latest => {
                let migrations = match &config.migrations {
                    Some(migrations) => migrations,
                    None => return Err(ErrorType::MissingConfigDefinition("Missing migrations definition".into()))
                };

                let location = util::standardise_path(&migrations).expect("Unable to standardise path.");

                let migrations = reader::find_migration_files(location.clone())
                    .expect("no migrations found");

                let mut driver = Driver::from_config(config);

                let report = driver
                    .migrate(migrations)
                    .await
                    .expect("could not generate report");

                info!("{}", report);
            }
        }
    }

    Ok(())
}
