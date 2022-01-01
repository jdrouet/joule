use crate::record::Record;
use clap::Parser;
use powercap::PowerCap;
use std::convert::TryFrom;
use std::path::PathBuf;

#[derive(Parser)]
pub struct SnapshotCommand {
    /// Path to powercap folder
    #[clap(long, default_value = "/sys/class/powercap")]
    powercap_path: PathBuf,
    /// Path to write the snapshot file
    output: PathBuf,
}

impl SnapshotCommand {
    pub fn execute(self) -> Result<(), String> {
        let pcap = PowerCap::try_from(self.powercap_path)
            .map_err(|err| format!("unable to create powercap instance from path: {:?}", err))?;
        let snapshot = pcap
            .intel_rapl
            .snapshot()
            .map_err(|err| format!("unable to read powercap data: {:?}", err))?;
        let record = Record::from(snapshot);
        let record = serde_json::to_string(&record)
            .map_err(|err| format!("unable to convert record to json: {:?}", err))?;
        std::fs::write(&self.output, &record)
            .map_err(|err| format!("unable to write json to file: {:?}", err))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SnapshotCommand;
    use powercap::mock::MockBuilder;
    use std::path::PathBuf;
    use temp_dir::TempDir;

    impl SnapshotCommand {
        pub fn new(powercap_path: PathBuf, output: PathBuf) -> Self {
            Self {
                powercap_path,
                output,
            }
        }
    }

    #[test]
    fn execute() {
        let root = TempDir::new().unwrap();
        let pcap = root.path().join("pcap");
        MockBuilder::default().build(&pcap).unwrap();
        let output = root.path().join("output.json");
        let cmd = SnapshotCommand::new(pcap, output);
        cmd.execute().unwrap();
    }
}
