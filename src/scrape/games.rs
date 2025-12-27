use crate::error::Result;
use crate::html;
use reqwest::{Client, Url};
use scraper::Selector;

pub async fn scrape(client: &Client, jam_url: Url) -> Result<Vec<Url>> {
    let html = html::get(client, jam_url).await?;

    let selector = "#games .single-game > a";
    let selector = Selector::parse(selector).unwrap();
    let mut game_links = Vec::new();

    for element in html.select(&selector) {
        let href = html::extract::href(element)?;
        game_links.push(href);
    }

    // This game list is allowed to be empty:
    // There could potentially be an ongoing game jam with no results yet.

    Ok(game_links)
}
