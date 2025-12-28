mod cli;
mod error;
mod html;
mod io;
mod scrape;
mod url;
mod zip;

use clap::Parser;
use std::process::exit;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    println!("Started program");
    if let Err(e) = scrape::data_files(args).await {
        eprintln!("{}", e.chain());
        exit(1);
    }
}
