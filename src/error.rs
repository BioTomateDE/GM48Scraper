use std::fmt;

#[derive(Debug)]
pub struct ErrorReport<'a>(&'a anyhow::Error);

impl fmt::Display for ErrorReport<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const RED_BOLD: &str = "\x1B[31;1m";
        const RESET_BOLD: &str = "\x1B[22m";
        const RESET: &str = "\x1B[0m";

        let error = self.0;
        write!(f, "{RED_BOLD}Error:{RESET_BOLD} {error}")?;
        for e in error.chain().skip(1) {
            write!(f, "\n> {e}")?;
        }
        f.write_str(RESET)
    }
}

pub fn print_error(error: &anyhow::Error) {
    eprintln!("{}", ErrorReport(error));
}
