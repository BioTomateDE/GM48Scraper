use crate::error::{Context, Result};
use crate::scrape::CLIENT;
use crate::{archive, html};
use colored_print::{ceprintln, cprintln};
use reqwest::{Response, Url};
use scraper::Selector;
use std::fs;
use std::path::Path;

pub async fn windows_url(game_url: Url) -> Result<Option<Url>> {
    let html = html::get(game_url).await?;

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

pub async fn game(download_url: Url, file_path: &Path) -> Result<()> {
    cprintln!("Downloading game %d^{download_url}");
    let ctx = || format!("downloading game from {download_url}");

    let resp = CLIENT
        .get(download_url.clone())
        .send()
        .await
        .and_then(Response::error_for_status)
        .with_context(ctx)?;
    let bytes = resp
        .bytes()
        .await
        .context("getting response bytes")
        .with_context(ctx)?;

    let size = bytes.len();
    let human_size = humansize::format_size(size, humansize::BINARY);
    cprintln!("%B:Downloaded %u^{human_size}%_^ ({size} bytes)");

    let task = tokio::task::spawn_blocking(move || archive::extract::find_data_file(&bytes));
    let result = task
        .await
        .context("extracting downloaded ZIP archive")
        .with_context(ctx)?;
    match result {
        Ok(data_file_content) => {
            fs::write(file_path, data_file_content)
                .with_context(|| format!("writing extracted data file to {file_path:?}"))
                .with_context(ctx)?;
        }
        Err(err) => {
            let err = err.chain();
            ceprintln!("Could not find datafile for download {download_url}:\n%R:{err}");
        }
    }
    Ok(())
}
