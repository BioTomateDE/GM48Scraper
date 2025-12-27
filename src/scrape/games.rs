use crate::error::Result;
use crate::html;
use reqwest::Url;
use scraper::Selector;

pub async fn scrape(jam_url: Url) -> Result<Vec<Url>> {
    println!("Scraping games from {jam_url}");
    let html = html::get(jam_url.clone()).await?;

    let selector = "#games .single-game > a";
    let selector = Selector::parse(selector).unwrap();
    let mut game_links = Vec::new();

    for element in html.select(&selector) {
        let href = html::extract::href(element)?;
        game_links.push(href);
    }

    // This game list is allowed to be empty:
    // There could potentially be an ongoing game jam with no results yet.

    println!("Got {} games from {}", game_links.len(), jam_url);
    Ok(game_links)
}
