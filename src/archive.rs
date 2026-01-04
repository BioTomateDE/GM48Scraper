use colored_print::ceprintln;

use crate::error::{Context, Result, bail};
use std::{fmt::Display, io::Cursor};

/// Taken from <https://forum.gamemaker.io/index.php?threads/summary-of-gms-file-extensions.82460/>
/// and extended a little bit.
const KNOWN_GM_EXTENSIONS: &[&str] = &[
    "gm81", "gmez", "gml", "gmk", "gmx", "gmz", "yy", "yymp", "yymps", "yyp", "yyz",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Zip,
    Rar,
    SevenZip,
    PackedExe,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Kind::Zip => "ZIP",
            Kind::Rar => "RAR",
            Kind::SevenZip => "7-Zip",
            Kind::PackedExe => "packed .exe",
        };
        write!(f, "{string}")
    }
}

/// Extract the GameMaker data file from an archive in memory.
pub fn find_data_file(archive_data: &[u8], kind: Kind) -> Result<Vec<u8>> {
    let files: Vec<String> = list_files(archive_data)?;

    for file_path in &files {
        let filename = get_filename(file_path);
        let extension = get_extension(filename).to_ascii_lowercase();

        // Hey, that's the one we want!
        if filename == "data.win" {
            return extract_file(archive_data, file_path);
        }

        if KNOWN_GM_EXTENSIONS.contains(&extension.as_str()) {
            // Uploader has a skill issue, nothing I can do about that
            bail!(
                "Found incorrectly uploaded GameMaker project \
                in Windows download (detected by file {filename:?})"
            );
        }

        if kind == Kind::PackedExe && filename == "data" {
            // This is a common "filename" for packed installers.
            // Installers are useless to me; I need runners.
            bail!("Packed .exe file is an installer instead of a runner");
        }
    }

    // Couldn't find data.win, try finding inner archives
    for file_path in &files {
        let extension = get_extension(file_path).to_ascii_lowercase();
        let kind = match extension.as_str() {
            "zip" => Kind::Zip,
            "rar" => Kind::Rar,
            "7z" => Kind::SevenZip,
            "exe" => Kind::PackedExe,
            _ => continue,
        };
        let inner_archive = extract_file(archive_data, file_path)?;
        return find_data_file(&inner_archive, kind)
            .with_context(|| format!("extracting inner {kind} archive {file_path:?}"));
    }

    // Couldn't find data.win or inner archive.
    // Print archive structure for debugging and throw.
    print_structure(&files, kind);
    bail!("Could not find data.win file in archive");
}

fn list_files(archive_data: &[u8]) -> Result<Vec<String>> {
    compress_tools::list_archive_files(&mut Cursor::new(archive_data))
        .context("listing archive files")
}

fn extract_file(archive_data: &[u8], file_path: &str) -> Result<Vec<u8>> {
    let mut cursor = Cursor::new(archive_data);
    let mut output = Vec::new();

    compress_tools::uncompress_archive_file(&mut cursor, &mut output, file_path)
        .with_context(|| format!("Failed to extract file {file_path:?}"))?;

    Ok(output)
}

#[must_use]
fn get_filename(file_path: &str) -> &str {
    last_part(file_path, '/')
}

#[must_use]
fn get_extension(filename: &str) -> &str {
    last_part(filename, '.')
}

#[must_use]
fn last_part(string: &str, delimiter: char) -> &str {
    string.split(delimiter).next_back().unwrap_or(string)
}

fn print_structure(files: &[String], kind: Kind) {
    ceprintln!("\n%b^========%M: {kind} Archive Structure%_: ========");
    for path in files {
        ceprintln!("%b:{path:?}");
    }
    eprintln!();
}
