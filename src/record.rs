use powercap::{IntelRaplSnapshot, SocketSnapshot};
use std::ops::Sub;
use std::time::SystemTime;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RecordDifference {
    /// Measurement duration in seconds
    pub duration: f64,
    /// Consumed energy in micro joules
    pub energy: u64,
    // TODO add the difference by socket and domains
}

impl RecordDifference {
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|err| format!("unable to convert to json: {:?}", err))
    }

    pub fn to_key_value(&self) -> String {
        format!("duration={} energy={}", self.duration, self.energy)
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Record {
    time: SystemTime,
    sockets: Vec<SocketSnapshot>,
}

impl Record {
    fn total_energy(&self) -> u64 {
        self.sockets
            .iter()
            .map(|item| {
                item.domains
                    .iter()
                    .fold(item.energy, |res, domain| res + domain.energy)
            })
            .sum()
    }
}

impl From<IntelRaplSnapshot> for Record {
    fn from(cap: IntelRaplSnapshot) -> Self {
        Self {
            time: SystemTime::now(),
            sockets: cap.sockets,
        }
    }
}

impl Sub for Record {
    type Output = Result<RecordDifference, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        // make sure that self is taken before rhs
        let duration = rhs
            .time
            .duration_since(self.time)
            .map_err(|err| format!("unable to compute duration between records: {:?}", err))?
            .as_secs_f64();
        let energy = rhs.total_energy() - self.total_energy();

        Ok(RecordDifference { duration, energy })
    }
}
