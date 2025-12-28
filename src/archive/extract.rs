use crate::error::{Context, Result, bail};
use std::io::Cursor;

/// Extract the GameMaker data file from an archive in memory.
pub fn find_data_file(archive_data: &[u8]) -> Result<Vec<u8>> {
    let file_list = compress_tools::list_archive_files(&mut Cursor::new(archive_data))
        .context("Failed to list archive files")?;

    for file_path in &file_list {
        let filename = get_filename(file_path);
        let extension = get_extension(filename);

        // Hey, that's the one we want!
        if filename == "data.win" {
            return extract_file(archive_data, file_path);
        }

        // Uploader has a skill issue, nothing I can do about that
        if extension == "yyp" {
            bail!("Found incorrectly uploaded GameMaker project in Windows download");
        }

        // This is a common "filename" for packed installers.
        // Installers are useless to me; I need runners.
        if filename == "data" {
            bail!("Found packed exe installer; useless for extracting data.win");
        }
    }

    // Couldn't find data.win, try finding packed EXEs
    for file_path in &file_list {
        if get_extension(file_path) == "exe" {
            let exe_archive = extract_file(archive_data, file_path)?;
            return find_data_file(&exe_archive)
                .with_context(|| format!("extracting packed exe file {file_path:?}"));
        }
    }

    // Nothing worked, print directory for debugging and exit.
    super::print::archive_structure(archive_data);
    bail!("Could not find data.win or packed .exe file in archive");
}

fn extract_file(archive_data: &[u8], file_path: &str) -> Result<Vec<u8>> {
    let mut cursor = Cursor::new(archive_data);
    let mut output = Vec::new();

    compress_tools::uncompress_archive_file(&mut cursor, &mut output, file_path)
        .with_context(|| format!("Failed to extract file {file_path:?}"))?;

    Ok(output)
}

fn get_filename(file_path: &str) -> &str {
    file_path.split('/').next_back().unwrap_or(file_path)
}

fn get_extension(filename: &str) -> &str {
    filename.split('.').next_back().unwrap_or(filename)
}
