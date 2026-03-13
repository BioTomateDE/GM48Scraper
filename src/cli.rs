use clap::Parser;
use std::{num::NonZeroU32, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory the data files will be downloaded to.
    #[arg(default_value = "gm48_datafiles")]
    pub directory: PathBuf,

    /// How many requests can be sent at once.
    ///
    /// Higher values may speed up the program,
    /// but at a certain point the server will block requests due to spam.
    #[arg(short, long, default_value = "6")]
    pub jobs: NonZeroU32,
}

#[must_use]
pub fn parse() -> Args {
    Args::parse()
}
