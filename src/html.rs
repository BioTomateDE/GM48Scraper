use crate::scrape::CLIENT;

use anyhow::Context;
use anyhow::Result;
use reqwest::Response;
use reqwest::Url;
use scraper::ElementRef;
use scraper::Html;

/// Sends a GET request to the specified URL and then extracts the HTML of the response.
pub async fn get_html(url: Url) -> Result<Html> {
    let text: String = CLIENT
        .get(url.clone())
        .send()
        .await
        .and_then(Response::error_for_status)
        .with_context(|| format!("Failed to send GET request to {url}"))?
        .text()
        .await
        .with_context(|| format!("Failed to get text response body of GET request to {url}"))?;
    Ok(Html::parse_document(&text))
}

/// Gets the `href` of an `<a>` HTML tag and parses it as a URL.
pub fn extract_href(element: ElementRef) -> Result<Url> {
    let href: &str = element.attr("href").context("Missing href in HTML hyperlink tag")?;
    Url::parse(href).with_context(|| format!("Invalid href URL in HTML hyperlink tag: {href:?}"))
}
