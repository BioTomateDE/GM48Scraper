#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

// It doesn't know that GameMaker is a real term that doesn't need backticks.
#![allow(clippy::doc_markdown)]

// Out of my control.
#![allow(clippy::multiple_crate_versions)]

mod archive;
mod cli;
mod error;
mod filename;
mod html;
mod scrape;
mod url;

#[tokio::main]
async fn main() {
    let args = cli::parse();
    println!("Let the programme commence forth.");

    if let Err(error) = scrape::scrape_data_files(args).await {
        error.print_exit();
    }
}
