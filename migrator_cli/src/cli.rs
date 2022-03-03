use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "migrator", about = "Migrate your database")]
pub enum Opts {
    // Setup the migration tool
    Setup(Setup),

    Migrate(Migrate),
}

#[derive(Debug, StructOpt)]
pub enum Migrate {
    // Create a migration
    Make(MigrateMake),

    // Migrate to latest
    Latest,

    // Rollback latest migration
    Rollback,
}

#[derive(Debug, StructOpt)]
pub struct MigrateMake {
    #[structopt(short, long, help = "Name of the migration")]
    pub name: String,
}

#[derive(Debug, StructOpt)]
pub enum Setup {
    Init,

    Set(Set),

    View,
}

#[derive(Debug, StructOpt)]
pub struct Set {
    #[structopt(short, long, help = "Url for database")]
    pub uri: Option<String>,

    #[structopt(short, long, help = "Path to migrations")]
    pub migrations: Option<String>,
}

pub fn parse() -> Opts {
    return Opts::from_args();
}
