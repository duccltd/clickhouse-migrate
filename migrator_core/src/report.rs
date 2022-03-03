use crate::dbl::MigrationFile;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct ExecutionReport {
    ran_migrations: Vec<MigrationFile>,
}

impl ExecutionReport {
    pub fn new(ran_migrations: Vec<MigrationFile>) -> Self {
        ExecutionReport {
            ran_migrations
        }
    }
}

impl std::fmt::Display for ExecutionReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for migration in &self.ran_migrations {
            writeln!(f, "{}", migration)?;
        }
        writeln!(f, "{} migrations", &self.ran_migrations.len())
    }
}