mod archive;
mod cli;
mod error;
mod html;
mod scrape;
mod url;

use clap::Parser;
use colored_print::ceprintln;
use std::process::exit;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    println!("Started program");
    if let Err(e) = scrape::data_files(args).await {
        ceprintln!("Error: %R:{}", e.chain());
        exit(1);
    }
}
