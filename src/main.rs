mod command;
mod record;

use clap::Parser;

fn main() {
    if let Err(err) = command::Command::parse().execute() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
