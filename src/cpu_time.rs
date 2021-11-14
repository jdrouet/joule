use procfs::{KernelStats, ProcResult};
use std::convert::TryFrom;
use std::fs::File;
use std::path::PathBuf;

pub struct CpuTimeReader {
    #[allow(dead_code)]
    root: File,
}

impl TryFrom<PathBuf> for CpuTimeReader {
    type Error = String;

    fn try_from(root: PathBuf) -> Result<Self, String> {
        let root = File::open(root).map_err(|err| err.to_string())?;
        Ok(Self { root })
    }
}

impl CpuTimeReader {
    pub fn read(&self) -> ProcResult<KernelStats> {
        // KernelStats::from_reader(&self.root)
        KernelStats::new()
    }
}
