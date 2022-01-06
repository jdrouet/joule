use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
pub enum Exporter {
    Stdout,
    File {
        /// Path to the file to be created
        output: PathBuf,
    },
}

impl Exporter {
    pub fn writer(&self) -> Result<Box<dyn Write>, String> {
        match self {
            Self::Stdout => Ok(Box::new(std::io::stdout())),
            Self::File { output } => {
                let file = File::create(output)
                    .map_err(|err| format!("unable to create output file: {:?}", err))?;
                Ok(Box::new(file))
            }
        }
    }
}
