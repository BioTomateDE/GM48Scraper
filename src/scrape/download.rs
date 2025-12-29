use crate::archive;
use crate::error::{Context, Result};
use crate::scrape::CLIENT;
use colored_print::{ceprintln, cprintln};
use reqwest::{StatusCode, Url};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

pub async fn game(url: Url, file_path: &Path) -> Result<()> {
    let ctx = || format!("downloading game from {url}");

    let resp = CLIENT.get(url.clone()).send().await.with_context(ctx)?;
    if resp.status() == StatusCode::NOT_FOUND {
        cprintln!("%y:Skipping download for %_:{url}%y:: %Y:Game does not have a Windows download");
        return Ok(());
    }
    resp.error_for_status_ref().with_context(ctx)?;

    let archive_data = resp
        .bytes()
        .await
        .context("getting response bytes")
        .with_context(ctx)?;

    let size = archive_data.len();
    let human_size = humansize::format_size(size, humansize::BINARY);
    cprintln!("Downloaded %u^{human_size}%_^ ({size} bytes) from %d^{url}");

    let task = tokio::task::spawn_blocking(move || {
        archive::find_data_file(&archive_data, archive::Kind::Zip)
    });
    let result = task
        .await
        .context("extracting downloaded ZIP archive")
        .with_context(ctx)?;

    match result {
        Ok(data_file_content) => {
            fs::write(file_path, data_file_content)
                .with_context(|| format!("writing extracted data file to {file_path:?}"))
                .with_context(ctx)?;

            let name = file_path
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap_or("<unknown>");
            cprintln!("%G:Sucessfully extracted data file to %g:{name}");
        }
        Err(err) => {
            let err = err.chain();
            ceprintln!("Could not find data.win file for download {url}:\n%R:{err}");
        }
    }
    Ok(())
}
