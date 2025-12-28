use crate::error::Context;
use crate::scrape::CLIENT;
use reqwest::{Response, Url};
use scraper::Html;

pub mod extract;

/// Send a GET request to the specified URL
/// and then extract the HTML of the response.
pub async fn get(url: Url) -> crate::error::Result<Html> {
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
