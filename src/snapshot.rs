use crate::cpu_energy::CpuEnergyReader;
use crate::cpu_time::CpuTimeReader;
use crate::process::{process_time, ProcessReader};
use powercap::SocketSnapshot;
use procfs::process::Process;
use procfs::{CpuTime, KernelStats};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Snapshot {
    pub time: SystemTime,
    pub cpu_energy: Vec<SocketSnapshot>,
    pub cpu_time: KernelStats,
    pub processes: Vec<Process>,
}

fn cpu_time(time: &CpuTime) -> u64 {
    time.user
        + time.nice
        + time.system
        + time.guest.unwrap_or_default()
        + time.guest_nice.unwrap_or_default()
}

impl Snapshot {
    pub fn total_energy(&self) -> u64 {
        self.cpu_energy.iter().fold(0, |res, socket| {
            res + socket
                .domains
                .iter()
                .fold(socket.energy, |res, domain| res + domain.energy)
        })
    }

    pub fn total_cpu_time(&self) -> u64 {
        cpu_time(&self.cpu_time.total)
    }

    pub fn processes_time(&self) -> HashMap<i32, u64> {
        self.processes
            .iter()
            .map(|p| (p.pid, process_time(p)))
            .collect()
    }

    pub fn print(&self) {
        let processes_time: u64 = self.processes_time().values().sum();
        println!(
            "time={}s total_cpu_time={} total_energy={}µJ processes_time={}",
            self.time.duration_since(UNIX_EPOCH).unwrap().as_secs_f64(),
            self.total_cpu_time(),
            self.total_energy(),
            processes_time
        );
    }
}

pub struct SnapshotReader {
    pub cpu_energy: CpuEnergyReader,
    pub cpu_time: CpuTimeReader,
    pub process: ProcessReader,
}

impl SnapshotReader {
    pub fn read(&self) -> Result<Snapshot, String> {
        Ok(Snapshot {
            time: SystemTime::now(),
            cpu_energy: self.cpu_energy.read().map_err(|err| format!("{:?}", err))?,
            cpu_time: self.cpu_time.read().map_err(|err| format!("{:?}", err))?,
            processes: self.process.read().map_err(|err| format!("{:?}", err))?,
        })
    }
}
