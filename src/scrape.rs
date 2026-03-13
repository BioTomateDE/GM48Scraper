mod download;
mod game_jams;
mod games;

use crate::error::{Context, Result};
use crate::filename::sanitize_filename;
use crate::scrape::games::scrape_games;
use crate::{cli, url};
use colored_print::cprintln;
use download::download_game;
use futures::stream::{self, StreamExt};
use reqwest::{Client, Url};
use std::path::PathBuf;
use std::sync::LazyLock;
use url::extract_meta;

/// A reusable `reqwest` client instance.
pub static CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

pub async fn scrape_data_files(args: cli::Args) -> Result<()> {
    std::fs::create_dir_all(&args.directory).context("creating output directory")?;

    let game_jam_urls = game_jams::scrape()
        .await
        .context("getting list of game jams")?;
    cprintln!("%C:Got {} game jams.", game_jam_urls.len());

    let batch_size = u32::from(args.jobs) as usize;

    // TODO: handle errors?
    let mut game_urls: Vec<Url> = stream::iter(game_jam_urls)
        .map(scrape_games)
        .buffer_unordered(batch_size)
        .filter_map(|result| async { result.ok() }) // this ignores errors
        .flat_map(stream::iter)
        .collect()
        .await;
    game_urls.sort();
    cprintln!("%C:Got %b^{}%_^ games in total.", game_urls.len());

    let _: Vec<()> = stream::iter(game_urls)
        .map(|url| handle_game_wrapper(url, args.directory.clone()))
        .buffer_unordered(batch_size)
        .collect()
        .await;

    cprintln!("%G:%b^All games downloaded!");
    Ok(())
}

async fn handle_game_wrapper(url: Url, dir: PathBuf) {
    if let Err(e) = handle_game(url, dir).await {
        e.print();
    }
}

async fn handle_game(game_url: Url, dir: PathBuf) -> Result<()> {
    let (jam, game) = extract_meta(&game_url)?;
    let jam = sanitize_filename(jam);
    let game = sanitize_filename(game);
    let filename = format!("{jam}_{game}.win");
    let path = dir.join(filename);

    if path.exists() {
        //cprintln!("%y:Skipping download for {url}: %Y:File already exists");
        return Ok(());
    }

    let url = format!("{game_url}/download/windows");
    download_game(&url, &path).await.with_context(|| format!("downloading game from {url}"))
}
