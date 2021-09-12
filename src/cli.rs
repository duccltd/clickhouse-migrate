use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "fifty")]
pub struct Opts {
    #[clap(short, long, default_value = "clickhouse")]
    pub driver: String,

    #[clap(short, long, default_value = "http://localhost:8123")]
    pub url: String,

    #[clap(short, long)]
    pub migrations: String,
}

pub fn parse() -> Opts {
    return Opts::parse();
}