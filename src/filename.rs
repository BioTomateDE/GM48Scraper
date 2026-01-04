use std::borrow::Cow;

use colored_print::ceprintln;

#[must_use]
pub fn sanitize(filename: &str) -> String {
    // URL-decode filename since it was extracted from a URL.
    // I don't want ten billion percents in my filename.
    let filename = urlencoding::decode(filename).unwrap_or_else(|e| {
        ceprintln!("%Y:%b^WARNING%_^: Could not URL-decode filename {filename:?}: {e}");
        Cow::Borrowed(filename)
    });

    let mut options = sanitise_file_name::Options::DEFAULT;

    // Replace with '-' instead of '_' since underscores are
    // used as a delimiter between game jam and game name.
    options.replace_with = Some('-');

    // If the entire name is empty after sanitization, replace it with this.
    options.six_measures_of_barley = "this-game-name-is-entirely-invalid";

    sanitise_file_name::sanitize_with_options(&filename, &options)
}
