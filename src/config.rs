use crate::cpu_energy::CpuEnergyReader;
use crate::cpu_time::CpuTimeReader;
use crate::process::ProcessReader;
use crate::snapshot::SnapshotReader;
use clap::{App, AppSettings, Arg};
use std::convert::TryFrom;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub struct Config {
    process_path: PathBuf,
    powercap_path: PathBuf,
    trailing: Vec<String>,
}

impl Config {
    pub fn parse() -> Self {
        let matches = App::new(clap::crate_name!())
            .setting(AppSettings::TrailingVarArg)
            .about(clap::crate_description!())
            .license(clap::crate_license!())
            .version(clap::crate_version!())
            .author(clap::crate_authors!("\n"))
            .arg(
                Arg::new("process-path")
                    .long("process-path")
                    .takes_value(true)
                    .env("PROCESS_PATH")
                    .default_value("/proc"),
            )
            .arg(
                Arg::new("powercap-path")
                    .long("powercap-path")
                    .takes_value(true)
                    .env("POWERCAP_PATH")
                    .default_value("/sys/class/powercap"),
            )
            .arg(Arg::from("<cmd>... 'The command to execute'"))
            .get_matches();

        let process_path: String = matches.value_of_t_or_exit("process-path");
        let powercap_path: String = matches.value_of_t_or_exit("powercap-path");
        let trailing: Vec<String> = matches
            .values_of("cmd")
            .unwrap()
            .into_iter()
            .map(String::from)
            .collect();

        Self {
            process_path: PathBuf::from(process_path),
            powercap_path: PathBuf::from(powercap_path),
            trailing,
        }
    }
}

impl Config {
    pub fn execute(&self) {
        let cmd = self.trailing.as_slice().first().unwrap();
        let args = self
            .trailing
            .iter()
            .enumerate()
            .filter(|(i, _)| *i > 0)
            .map(|(_, value)| value)
            .collect::<Vec<_>>();
        Command::new(cmd)
            .args(args)
            .stdout(Stdio::inherit())
            .output()
            .expect("failed to execute command...");
    }

    pub fn process_reader(&self) -> ProcessReader {
        ProcessReader::from(self.process_path.clone())
    }

    pub fn cpu_time_reader(&self) -> CpuTimeReader {
        CpuTimeReader::try_from(self.process_path.join("stat")).unwrap()
    }

    pub fn cpu_energy_reader(&self) -> CpuEnergyReader {
        CpuEnergyReader::try_from(self.powercap_path.clone()).unwrap()
    }

    pub fn snapshot_reader(&self) -> SnapshotReader {
        SnapshotReader {
            cpu_energy: self.cpu_energy_reader(),
            cpu_time: self.cpu_time_reader(),
            process: self.process_reader(),
        }
    }
}
