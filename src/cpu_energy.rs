use powercap::{PowerCap, SocketSnapshot};
use std::convert::TryFrom;
use std::path::PathBuf;

pub struct CpuEnergyReader {
    pcap: PowerCap,
}

impl TryFrom<PathBuf> for CpuEnergyReader {
    type Error = powercap::BuildError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let pcap = PowerCap::try_from(value)?;
        Ok(Self { pcap })
    }
}

impl CpuEnergyReader {
    pub fn read(&self) -> Result<Vec<SocketSnapshot>, powercap::ReadError> {
        self.pcap.intel_rapl.snapshot().map(|res| res.sockets)
    }
}
