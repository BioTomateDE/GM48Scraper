use clap::Parser;
use std::{num::NonZeroU32, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(default_value = "gm48_datafiles")]
    /// Directory the data files will be downloaded to.
    pub directory: PathBuf,

    #[arg(short, long, default_value = "6")]
    /// How many requests can be sent at once.
    ///
    /// Higher values may speed up the program,
    /// but at a certain point the server will block requests due to spam.
    pub jobs: NonZeroU32,
}

#[must_use]
pub fn parse() -> Args {
    Args::parse()
}
