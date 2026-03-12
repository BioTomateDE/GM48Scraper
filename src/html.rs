use crate::error::{Context, Result};
use crate::scrape::CLIENT;
use reqwest::{Response, Url};
use scraper::{ElementRef, Html};

/// Send a GET request to the specified URL
/// and then extract the HTML of the response.
pub async fn get_html(url: Url) -> Result<Html> {
    let text: String = CLIENT
        .get(url.clone())
        .send()
        .await
        .and_then(Response::error_for_status)
        .with_context(|| format!("sending GET request to {url}"))?
        .text()
        .await
        .with_context(|| format!("getting text response body of GET request to {url}"))?;
    let html = Html::parse_document(&text);
    Ok(html)
}

/// Get the Href URL of an `<a>` HTML node.
pub fn extract_href(element: ElementRef) -> Result<Url> {
    let href: &str = element
        .attr("href")
        .context("Link node <a> does not contain href")?;
    let url = Url::parse(href)?;
    Ok(url)
}
