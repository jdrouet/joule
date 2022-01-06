use crate::record::Record;

#[derive(clap::ArgEnum, Clone)]
pub enum Formatter {
    Json,
    KeyValue,
}

impl Formatter {
    pub fn format(&self, record: &Record) -> String {
        match self {
            Self::Json => format!("{{\"time\":{},\"energy\":{}}}", record.time, record.energy),
            Self::KeyValue => format!("time={} energy={}", record.time, record.energy),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Formatter;
    use crate::record::Record;

    #[test]
    fn to_json() {
        let record = Record {
            time: 42,
            energy: 101,
        };
        let json = Formatter::Json.format(&record);
        assert_eq!(json, "{\"time\":42,\"energy\":101}");
    }

    #[test]
    fn to_key_value() {
        let record = Record {
            time: 42,
            energy: 101,
        };
        let json = Formatter::KeyValue.format(&record);
        assert_eq!(json, "time=42 energy=101");
    }
}
