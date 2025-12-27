use reqwest::{Client, Url};
use scraper::Html;

pub mod extract;

/// Send a GET request to the specified URL
/// and then extract the HTML of the response.
pub async fn get(client: &Client, url: Url) -> crate::error::Result<Html> {
    let text: String = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    let html = Html::parse_document(&text);
    Ok(html)
}
