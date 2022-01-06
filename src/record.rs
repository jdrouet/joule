use powercap::{IntelRaplSnapshot, SocketSnapshot};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Record {
    /// Number of nanoseconds since linux epoch
    pub time: u128,
    /// Consumed energy since the computer boot in micro joules
    pub energy: u64,
}

fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("unable to compute time since epoch")
        .as_nanos()
}

fn total_energy(sockets: &Vec<SocketSnapshot>) -> u64 {
    sockets
        .iter()
        .map(|item| {
            item.domains
                .iter()
                .fold(item.energy, |res, domain| res + domain.energy)
        })
        .sum()
}

impl From<IntelRaplSnapshot> for Record {
    fn from(cap: IntelRaplSnapshot) -> Self {
        Self {
            time: now(),
            energy: total_energy(&cap.sockets),
        }
    }
}
