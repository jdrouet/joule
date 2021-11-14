use procfs::{process::Process, ProcResult};
use std::path::PathBuf;

pub fn process_time(process: &Process) -> u64 {
    process.stat.stime
        + process.stat.utime
        + process.stat.guest_time.unwrap_or_default()
        + process.stat.cguest_time.unwrap_or_default() as u64
        + process.stat.delayacct_blkio_ticks.unwrap_or_default()
        + process.stat.itrealvalue as u64
}

pub struct ProcessReader {
    root: PathBuf,
}

impl Default for ProcessReader {
    fn default() -> Self {
        Self::from(PathBuf::from("/proc"))
    }
}

impl From<PathBuf> for ProcessReader {
    fn from(root: PathBuf) -> Self {
        Self { root }
    }
}

impl ProcessReader {
    pub fn read(&self) -> ProcResult<Vec<Process>> {
        procfs::process::all_processes_with_root(&self.root)
    }
}
