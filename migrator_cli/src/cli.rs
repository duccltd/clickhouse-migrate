use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "migrator", about = "Migrate your database")]
pub enum Opts {
    #[structopt(name = "migrate")]
    Migrate {
        #[structopt(short, long, default_value = "clickhouse", help = "Database driver")]
        driver: String,

        #[structopt(short, long, default_value = "http://localhost:8123", help = "Url for database")]
        url: String,

        #[structopt(short, long, help = "Path to migrations")]
        migrations: String,
    }
}

pub fn parse() -> Opts {
    return Opts::from_args();
}