#![forbid(unsafe_code)]
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]
#![allow(
    // Out of my control.
    clippy::multiple_crate_versions,

    // False positives for `colored-print` macros.
    clippy::literal_string_with_formatting_args,
)]

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
        error.print();
        std::process::exit(1);
    }
}
