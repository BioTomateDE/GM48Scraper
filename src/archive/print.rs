use colored_print::ceprintln;
use std::io::Cursor;

/// Print an archive's files and directories for debugging.
pub fn archive_structure(archive_data: &[u8]) {
    let mut cursor = Cursor::new(archive_data);
    let Ok(file_list) = compress_tools::list_archive_files(&mut cursor) else {
        ceprintln!("%R:%b^Somehow failed to decompress the same archive again for error handling.");
        return;
    };

    ceprintln!("\n%b^======== %M: Archive Structure%_: ========");
    for path in file_list {
        ceprintln!("%b:{path:?}");
    }
    eprintln!();
}
