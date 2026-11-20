#![forbid(unsafe_code)]

mod archive;
mod error;
mod filename;
mod html;
mod scrape;
mod url;

use crate::error::print_error;
use crate::scrape::scrape_data_files;
use clap::Parser;
use std::num::NonZeroU8;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory the data files will be downloaded to.
    #[arg(short, long, default_value = "gm48-datafiles")]
    pub output_dir: PathBuf,

    /// How many requests can be sent at once.
    ///
    /// Higher values may speed up the program,
    /// but at a certain point the server will block requests due to spam.
    #[arg(short, long, default_value = "6")]
    pub jobs: NonZeroU8,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("Let the programme commence forth.");

    if let Err(error) = scrape_data_files(args).await {
        print_error(&error);
        std::process::exit(1);
    }

    #[cfg(windows)]
    let _ = std::process::Command::new("pause").status();
}
