mod config;
mod cpu_energy;
mod cpu_time;
mod process;
mod record;
mod snapshot;

use std::env::args;
use std::process::{Command, Stdio};

fn main() {
    let config = config::Config::default();
    let snapshot_reader = config.snapshot_reader();
    let mut args = args()
        .enumerate()
        .filter(|(index, _)| *index > 0)
        .map(|(_, value)| value)
        .collect::<Vec<String>>();
    println!("executing {:?}", args);
    let cmd = args.remove(0);
    let before = snapshot_reader.read().unwrap();
    Command::new(cmd)
        .args(args)
        .stdout(Stdio::inherit())
        .output()
        .expect("failed to execute command...");
    let after = snapshot_reader.read().unwrap();
    before.print();
    after.print();
    let record = record::Record::from((&before, &after));
    record.print();
}
