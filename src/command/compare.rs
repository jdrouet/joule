use crate::record::Record;
use clap::Parser;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(clap::ArgEnum, Clone)]
pub enum Format {
    Json,
    KeyValue,
}

#[derive(Parser)]
pub struct CompareCommand {
    /// Format for the output data
    #[clap(arg_enum, long, default_value = "key-value")]
    format: Format,
    /// Path to the first snapshot
    first: PathBuf,
    /// Path to the second snapshot
    second: PathBuf,
}

macro_rules! parse_json {
    ($path: expr, $name: expr) => {
        std::fs::File::open($path)
            .map_err(|err| format!(concat!("unable to open ", $name, " snapshot: {:?}"), err))
            .map(BufReader::new)
            .and_then(|reader| {
                serde_json::from_reader(reader).map_err(|err| {
                    format!(concat!("unable to parse ", $name, " snapshot: {:?}"), err)
                })
            })
    };
}

impl CompareCommand {
    pub fn execute(self) -> Result<(), String> {
        let first: Record = parse_json!(&self.first, "first")?;
        let second: Record = parse_json!(&self.second, "second")?;

        let diff = (first - second)?;
        let diff = match self.format {
            Format::Json => diff.to_json()?,
            Format::KeyValue => diff.to_key_value(),
        };

        println!("{}", diff);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{CompareCommand, Format};
    use crate::command::snapshot::SnapshotCommand;
    use powercap::mock::MockBuilder;
    use std::path::PathBuf;
    use temp_dir::TempDir;

    impl CompareCommand {
        pub fn new(first: PathBuf, second: PathBuf) -> Self {
            Self {
                format: Format::Json,
                first,
                second,
            }
        }
    }

    #[test]
    fn execute() {
        let root = TempDir::new().unwrap();
        let pcap = root.path().join("pcap-first");
        MockBuilder::default()
            .with_socket_energy_generator(Box::new(|_| 50))
            .with_domain_energy_generator(Box::new(|_, _| 10))
            .build(&pcap)
            .unwrap();
        let first = root.path().join("first.json");
        let cmd = SnapshotCommand::new(pcap, first.clone());
        cmd.execute().unwrap();
        let pcap = root.path().join("pcap-second");
        MockBuilder::default()
            .with_socket_energy_generator(Box::new(|_| 500))
            .with_domain_energy_generator(Box::new(|_, _| 100))
            .build(&pcap)
            .unwrap();
        let second = root.path().join("second.json");
        let cmd = SnapshotCommand::new(pcap, second.clone());
        cmd.execute().unwrap();
        let cmd = CompareCommand::new(first, second);
        cmd.execute().unwrap();
    }
}
