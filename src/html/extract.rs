use crate::error::{Context, Result};
use reqwest::Url;
use scraper::ElementRef;

/// Get the Href URL of an `<a>` HTML node.
pub fn href(element: ElementRef) -> Result<Url> {
    let href: &str = element
        .attr("href")
        .context("Link node <a> does not contain href")?;
    let url = Url::parse(href)?;
    Ok(url)
}
