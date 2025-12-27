use colored_print::ceprintln;
use std::io::{Read, Seek};
use zip::ZipArchive;

/// Print a ZIP Archive's files and directories for debugging.
pub fn archive_structure<T: Seek + Read>(archive: &mut ZipArchive<T>) {
    ceprintln!("\n%b^======== %M:ZIP Archive Structure%_: ========");
    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
        let name = file.name();

        let ty: &str = match (file.is_dir(), file.is_symlink()) {
            (true, true) => "dir symlink",
            (true, false) => "dir",
            (false, true) => "file symlink",
            (false, false) => "file",
        };

        ceprintln!("[{ty}] %b:{name:?}");
    }
    eprintln!();
}
