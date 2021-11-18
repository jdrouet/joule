mod config;
mod cpu_energy;
mod cpu_time;
mod process;
mod record;
mod snapshot;

fn main() {
    let config = config::Config::parse();
    let snapshot_reader = config.snapshot_reader();
    let before = snapshot_reader.read().unwrap();
    config.execute();
    let after = snapshot_reader.read().unwrap();
    before.print();
    after.print();
    let record = record::Record::from((&before, &after));
    record.print();
}
