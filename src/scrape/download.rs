use crate::archive;
use crate::error::print_error;
use crate::filename::display_filename;
use crate::scrape::CLIENT;
use anyhow::Context;
use anyhow::Result;
use colored_print::cprintln;
use reqwest::StatusCode;
use reqwest::Url;
use std::path::Path;

pub async fn download_game(raw_url: &str, file_path: &Path) -> Result<()> {
    let url = Url::parse(raw_url).with_context(|| format!("Invalid Game Download URL {raw_url:?}"))?;
    let resp = CLIENT.get(url).send().await?;
    if resp.status() == StatusCode::NOT_FOUND {
        cprintln!("%y:Skipping download for %_:{raw_url}%y:: %Y:No Windows download available");
        return Ok(());
    }
    _ = resp.error_for_status_ref()?;

    let archive_data = resp.bytes().await.context("Failed to get response bytes")?;
    let size: usize = archive_data.len();
    let human_size: String = humansize::format_size(size, humansize::BINARY);
    cprintln!("Downloaded %u^{human_size}%_^ ({size} bytes) from %d^{raw_url}");

    let task = tokio::task::spawn_blocking(move || archive::find_data_file(&archive_data, archive::Kind::Zip));
    let result: Result<Vec<u8>> = task.await.context("Failed to extract downloaded content")?;
    let data_file_content: Vec<u8> = match result {
        Ok(bytes) => bytes,
        Err(err) => {
            print_error(&err.context(format!("Could not find data.win file for download {raw_url}")));
            return Ok(());
        }
    };

    tokio::fs::write(file_path, data_file_content)
        .await
        .with_context(|| format!("Failed to write extracted data file to `{}`", file_path.display()))?;

    let name: &str = display_filename(file_path);
    cprintln!("%G:Successfully extracted data file to %g:{name}");
    Ok(())
}
