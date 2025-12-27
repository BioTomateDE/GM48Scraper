use crate::error::Result;
use crate::{html, zip};
use colored_print::cprintln;
use reqwest::{Client, Url};
use scraper::Selector;
use std::fs;
use std::path::Path;

pub async fn windows_url(client: &Client, game_url: Url) -> Result<Option<Url>> {
    let html = html::get(client, game_url).await?;

    let selector = "#download a.dropdown-item";
    let selector = Selector::parse(selector).unwrap();
    for element in html.select(&selector) {
        let text: String = element.text().collect();
        if text.trim() == "Windows" {
            let href = html::extract::href(element)?;
            return Ok(Some(href));
        }
    }

    Ok(None)
}

pub async fn game(client: &Client, download_url: Url, file_path: &Path) -> Result<()> {
    cprintln!("Downloading game %d^{download_url}");

    let resp = client.get(download_url).send().await?;
    resp.error_for_status_ref()?;
    let bytes = resp.bytes().await?;

    let size = bytes.len();
    let human_size = humansize::format_size(size, humansize::BINARY);
    cprintln!("%B:Downloaded %u^{human_size}%_^ ({size} bytes)");

    let result = tokio::task::spawn_blocking(move || zip::extract::data_file(&bytes)).await?;
    match result {
        Ok(data_file_content) => {
            fs::write(file_path, data_file_content)?;
        }
        Err(err) => {
            eprintln!("{}", err.chain());
        }
    }
    Ok(())
}
