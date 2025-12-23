mod scrape;
mod util;

use crate::scrape::{download_game, get_games, get_jams, get_windows_download_url};
use crate::util::{extract_meta_from_game_url, mkdir, sanitize_filename};
use anyhow::{Context, Result};
use reqwest::Client;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    let client = &Client::new();

    let dir = PathBuf::from("./gm48_datafiles"); // TODO dynamic, use clap (?)
    mkdir(&dir)?;

    let jams = get_jams(client)
        .await
        .context("Could not get list of Game Jams")?;
    println!("Got {} game jams", jams.len());

    for game_jam_url in jams {
        println!("Downloading games from {game_jam_url}");
        let games = get_games(client, game_jam_url)
            .await
            .context("Could not get list of games in Game Jam")?;
        println!("Got {} games", games.len());

        for game_url in games {
            let (jam, game) = extract_meta_from_game_url(&game_url)?;
            let filename = format!("{jam}_{game}.win");
            let filename = sanitize_filename(&filename);
            let path = dir.join(filename);

            if path.exists() {
                println!("Skipping download for {game_url}: File already exists");
                continue;
            }

            let download_url = get_windows_download_url(client, game_url.clone())
                .await
                .context("Could not get download URL for Windows")?;
            let Some(url) = download_url else {
                println!("Skipping download for {game_url}: Game does not have a Windows download");
                continue;
            };

            download_game(client, url, &path).await?;
        }
    }

    println!("All games downloaded!");
    Ok(())
}
