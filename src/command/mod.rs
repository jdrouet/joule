mod compare;
mod snapshot;

use clap::Parser;

#[derive(Parser)]
enum SubCommand {
    /// Compare two snapshots and print the result
    #[clap()]
    Compare(compare::CompareCommand),
    /// Creates a snapshot of the actual power consumption
    #[clap()]
    Snapshot(snapshot::SnapshotCommand),
}

impl SubCommand {
    fn execute(self) -> Result<(), String> {
        match self {
            Self::Compare(inner) => inner.execute(),
            Self::Snapshot(inner) => inner.execute(),
        }
    }
}

#[derive(Parser)]
#[clap(about, version, author)]
pub struct Command {
    #[clap(subcommand)]
    inner: SubCommand,
}

impl Command {
    pub fn execute(self) -> Result<(), String> {
        self.inner.execute()
    }
}
