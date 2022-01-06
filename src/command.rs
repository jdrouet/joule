use crate::exporter::Exporter;
use crate::format::Formatter;
use crate::record::Record;
use clap::Parser;
use powercap::PowerCap;
use std::convert::TryFrom;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser)]
#[clap(about, version, author)]
pub struct Command {
    /// When enabled multiple value will be taken
    #[clap(long)]
    watch: bool,
    /// Interval in seconds between each reacord
    #[clap(long, default_value = "5")]
    watch_interval: u64,
    /// Path to powercap folder
    #[clap(long, default_value = "/sys/class/powercap")]
    powercap_path: PathBuf,
    /// Format of the output
    #[clap(arg_enum, long, default_value = "json")]
    format: Formatter,
    /// Where to export the data
    #[clap(subcommand)]
    exporter: Exporter,
}

impl Command {
    fn powercap(&self) -> Result<PowerCap, String> {
        PowerCap::try_from(self.powercap_path.clone())
            .map_err(|err| format!("unable to create powercap instance from path: {:?}", err))
    }

    fn record(&self, pcap: &PowerCap) -> Result<Record, String> {
        pcap.intel_rapl
            .snapshot()
            .map(Record::from)
            .map_err(|err| format!("unable to read powercap data: {:?}", err))
    }

    fn single(&self) -> Result<(), String> {
        let pcap = self.powercap()?;
        let mut writer = self.exporter.writer()?;
        let record = self.record(&pcap)?;
        let payload = self.format.format(&record);
        writer
            .write_all(payload.as_bytes())
            .map_err(|err| format!("unable to write payload: {:?}", err))
    }

    fn watch(&self) -> Result<(), String> {
        let pcap = self.powercap()?;
        let mut writer = self.exporter.writer()?;
        let interval = Duration::from_secs(self.watch_interval);
        loop {
            let record = self.record(&pcap)?;
            let payload = self.format.format(&record);
            writer
                .write_all(payload.as_bytes())
                .and_then(|_| writer.write_all("\n".as_bytes()))
                .map_err(|err| format!("unable to write payload: {:?}", err))?;
            sleep(interval);
        }
    }

    pub fn execute(self) -> Result<(), String> {
        if self.watch {
            self.watch()
        } else {
            self.single()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Command;
    use crate::exporter::Exporter;
    use crate::format::Formatter;
    use powercap::mock::MockBuilder;
    use std::path::PathBuf;
    use temp_dir::TempDir;

    #[test]
    fn single_json_stdout() {
        let root = TempDir::new().unwrap();
        MockBuilder::default().build(root.path()).unwrap();
        let cmd = Command {
            watch: false,
            watch_interval: 5,
            powercap_path: PathBuf::from(root.path()),
            format: Formatter::Json,
            exporter: Exporter::Stdout,
        };
        cmd.single().unwrap();
    }

    #[test]
    fn single_kv_file() {
        let root = TempDir::new().unwrap();
        MockBuilder::default().build(root.path()).unwrap();
        let cmd = Command {
            watch: false,
            watch_interval: 5,
            powercap_path: PathBuf::from(root.path()),
            format: Formatter::Json,
            exporter: Exporter::File {
                output: root.path().join("output.json"),
            },
        };
        cmd.single().unwrap();
    }
}
