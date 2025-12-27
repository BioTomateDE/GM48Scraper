mod download;
mod game_jams;
mod games;

use crate::error::{Context, Result};
use crate::io::mkdir;
use crate::{cli, url};
use colored_print::cprintln;
use reqwest::Client;

pub async fn data_files(args: cli::Args) -> Result<()> {
    let client = &Client::new();

    mkdir(&args.directory)?;

    let jams = game_jams::scrape(client)
        .await
        .context("Could not get list of Game Jams")?;
    cprintln!("%d^Got {} game jams", jams.len());

    for game_jam_url in jams {
        cprintln!("%G:Downloading games from Game Jam {game_jam_url}");
        let games = games::scrape(client, game_jam_url)
            .await
            .context("Could not get list of games in Game Jam")?;
        cprintln!("%d^Got {} games", games.len());

        for game_url in games {
            let (jam, game) = url::extract_meta(&game_url)?;
            let game = urlencoding::decode(game)?;
            let filename = format!("{jam}_{game}.win");
            let path = args.directory.join(filename);

            if path.exists() {
                cprintln!("%y:Skipping download for {game_url}: %Y:File already exists");
                continue;
            }

            let download_url = download::windows_url(client, game_url.clone())
                .await
                .context("Could not get download URL for Windows")?;
            let Some(url) = download_url else {
                cprintln!(
                    "%y:Skipping download for %_:{game_url}%y:: %Y:Game does not have a Windows download"
                );
                continue;
            };

            download::game(client, url, &path).await?;
        }
    }

    cprintln!("%G:%b^All games downloaded!");
    Ok(())
}
