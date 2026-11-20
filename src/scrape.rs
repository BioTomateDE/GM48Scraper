mod download;
mod games;
mod jams;

use crate::error::print_error;
use crate::filename::sanitize_filename;
use crate::scrape::games::scrape_games;
use crate::url;
use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use colored_print::cprintln;
use download::download_game;
use futures::stream;
use futures::stream::StreamExt;
use reqwest::Client;
use reqwest::Url;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;
use url::extract_meta;

/// A reusable `reqwest` client instance.
pub static CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

/// Scrapes all data files of the GM48 game jam.
pub async fn scrape_data_files(args: crate::Args) -> Result<()> {
    create_dir(&args.output_dir)?;

    let game_jam_urls = jams::scrape().await.context("Failed fetching list of game jams")?;
    cprintln!("%C:Got {} game jams.", game_jam_urls.len());

    let batch_size = usize::from(args.jobs.get());

    // TODO: handle errors?
    let mut game_urls: Vec<Url> = stream::iter(game_jam_urls)
        .map(scrape_games)
        .buffer_unordered(batch_size)
        .filter_map(|result| async { result.ok() }) // this ignores errors
        .flat_map(stream::iter)
        .collect()
        .await;
    game_urls.sort_unstable();
    cprintln!("%C:Got %b^{}%_^ games in total.\n", game_urls.len());

    let _: Vec<()> = stream::iter(game_urls)
        .map(|url| handle_game(url, args.output_dir.clone()))
        .buffer_unordered(batch_size)
        .collect()
        .await;

    cprintln!("%G:%b^All games downloaded!");
    Ok(())
}

async fn handle_game(url: Url, dir: PathBuf) {
    if let Err(e) = handle_game_impl(url, dir).await {
        print_error(&e);
    }
}

async fn handle_game_impl(game_url: Url, dir: PathBuf) -> Result<()> {
    let (jam, game) = extract_meta(&game_url)?;
    let jam = sanitize_filename(jam);
    let game = sanitize_filename(game);
    let filename = format!("{jam}_{game}.win");
    let path = dir.join(filename);

    if path.exists() {
        // cprintln!("%y:Skipping download for {url}: %Y:File already exists");
        return Ok(());
    }

    let url = format!("{game_url}/download/windows");
    download_game(&url, &path)
        .await
        .with_context(|| format!("Failed to download game from {url}"))
}

fn create_dir(path: &Path) -> Result<()> {
    match std::fs::create_dir(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        Err(e) => Err(anyhow!("Failed to create directory `{}`", path.display()).context(e)),
    }
}
