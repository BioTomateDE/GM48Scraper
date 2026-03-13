#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
//
// Out of my control.
#![allow(clippy::multiple_crate_versions)]
//
// False positives for `colored-print` macros.
#![allow(clippy::literal_string_with_formatting_args)]
//
// It doesn't know that GameMaker is a real term that doesn't need backticks.
#![allow(clippy::doc_markdown)]
//
// `path.display()`? i ain't typing allat
#![allow(clippy::unnecessary_debug_formatting)]

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
