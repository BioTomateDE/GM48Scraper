mod download;
mod game_jams;
mod games;

use crate::error::{Context, Result};
use crate::{cli, url};
use colored_print::cprintln;
use futures::stream::{self, StreamExt};
use reqwest::{Client, Url};
use std::path::PathBuf;
use std::sync::LazyLock;

const BATCH_SIZE: usize = 6;
pub static CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

pub async fn data_files(args: cli::Args) -> Result<()> {
    std::fs::create_dir_all(&args.directory).context("creating output directory")?;

    let game_jam_urls = game_jams::scrape()
        .await
        .context("getting list of game jams")?;
    cprintln!("%C:Got {} game jams", game_jam_urls.len());

    let mut game_urls: Vec<Url> = stream::iter(game_jam_urls)
        .map(games::scrape)
        .buffer_unordered(BATCH_SIZE)
        .filter_map(|result| async { result.ok() })
        .flat_map(stream::iter)
        .collect()
        .await;
    game_urls.sort();
    cprintln!("%C:Got {} games in total", game_urls.len());

    let _: Vec<()> = stream::iter(game_urls)
        .map(|url| handle_game_wrapper(url, args.directory.clone()))
        .buffer_unordered(BATCH_SIZE) // max 5 concurrent
        .collect()
        .await;

    cprintln!("%G:%b^All games downloaded!");
    Ok(())
}

async fn handle_game_wrapper(url: Url, dir: PathBuf) {
    if let Err(e) = handle_game(url, dir).await {
        cprintln!("%R:{}", e.chain());
    }
}

async fn handle_game(url: Url, dir: PathBuf) -> Result<()> {
    let (jam, game) = url::extract_meta(&url)?;
    let game = urlencoding::decode(game)?;
    let filename = format!("{jam}_{game}.win");
    let path = dir.join(filename);

    if path.exists() {
        //cprintln!("%y:Skipping download for {url}: %Y:File already exists");
        return Ok(());
    }

    let download_url = download::windows_url(url.clone())
        .await
        .context("getting download URL for Windows")?;

    let Some(url) = download_url else {
        cprintln!("%y:Skipping download for %_:{url}%y:: %Y:Game does not have a Windows download");
        return Ok(());
    };

    download::game(url, &path).await
}
