use crate::cpu_energy::CpuEnergyReader;
use crate::cpu_time::CpuTimeReader;
use crate::process::ProcessReader;
use crate::snapshot::SnapshotReader;
use std::convert::TryFrom;
use std::path::PathBuf;

pub struct Config {
    process_root: PathBuf,
    powercap_root: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            process_root: PathBuf::from("/proc"),
            powercap_root: PathBuf::from("/sys/class/powercap"),
        }
    }
}

impl Config {
    pub fn process_reader(&self) -> ProcessReader {
        ProcessReader::from(self.process_root.clone())
    }

    pub fn cpu_time_reader(&self) -> CpuTimeReader {
        CpuTimeReader::try_from(self.process_root.join("stat")).unwrap()
    }

    pub fn cpu_energy_reader(&self) -> CpuEnergyReader {
        CpuEnergyReader::try_from(self.powercap_root.clone()).unwrap()
    }

    pub fn snapshot_reader(&self) -> SnapshotReader {
        SnapshotReader {
            cpu_energy: self.cpu_energy_reader(),
            cpu_time: self.cpu_time_reader(),
            process: self.process_reader(),
        }
    }
}
