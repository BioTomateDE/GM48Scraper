use crate::error::{Context, Result, bail};
use std::io::Cursor;
use zip::ZipArchive;

/// Extracts the GameMaker data file from a ZIP archive in memory.
pub fn data_file(data: &[u8]) -> Result<Vec<u8>> {
    let mut archive = ZipArchive::new(Cursor::new(data))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if file.is_dir() {
            continue;
        }

        let filename = file.name();
        let filename = filename.rsplit_once("/").map_or(filename, |x| x.1);
        if filename != "data.win" {
            continue;
        }

        let size: usize = file
            .size()
            .try_into()
            .map_err(|e| format!("File is too massive for this poor architecture: {e}"))
            .context("getting size of ZIP file")?;

        let mut content = Vec::with_capacity(size);
        std::io::copy(&mut file, &mut content)?;
        return Ok(content);
    }

    // TODO: handle SFX (self extracting exe)
    //       interface 7zip to decompress them?

    // Failed to find file, print directory for debugging and exit
    super::print::archive_structure(&mut archive);
    bail!("Could not find a data file in ZIP archive");
}
