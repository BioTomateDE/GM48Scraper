use crate::archive;
use crate::error::{Context, Result};
use crate::filename::display_filename;
use crate::scrape::CLIENT;
use colored_print::{ceprintln, cprintln};
use reqwest::{StatusCode, Url};
use std::fs;
use std::path::Path;

pub async fn download_game(raw_url: &str, file_path: &Path) -> Result<()> {
    let url = Url::parse(raw_url).context("Invalid Game Download URL {raw_url:?}")?;
    let resp = CLIENT.get(url).send().await?;
    if resp.status() == StatusCode::NOT_FOUND {
        cprintln!("%y:Skipping download for %_:{raw_url}%y:: %Y:No Windows download available");
        return Ok(());
    }
    resp.error_for_status_ref()?;

    let archive_data = resp.bytes().await.context("getting response bytes")?;
    let size: usize = archive_data.len();
    let human_size: String = humansize::format_size(size, humansize::BINARY);
    cprintln!("Downloaded %u^{human_size}%_^ ({size} bytes) from %d^{raw_url}");

    let task = tokio::task::spawn_blocking(move || {
        archive::find_data_file(&archive_data, archive::Kind::Zip)
    });
    let result: Result<Vec<u8>> = task.await.context("extracting downloaded ZIP archive")?;

    match result {
        Ok(data_file_content) => {
            fs::write(file_path, data_file_content)
                .with_context(|| format!("writing extracted data file to {file_path:?}"))?;

            let name: &str = display_filename(file_path);
            cprintln!("%G:Successfully extracted data file to %g:{name}");
        }
        Err(err) => {
            let err = err.chain();
            ceprintln!("Could not find data.win file for download {raw_url}:\n%R:{err}");
        }
    }
    Ok(())
}
