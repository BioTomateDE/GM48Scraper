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

    if let Err(error) = scrape::data_files(args).await {
        error.print_exit();
    }
}
