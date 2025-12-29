use crate::error::Result;
use crate::html;
use colored_print::cprintln;
use reqwest::Url;
use scraper::Selector;

pub async fn scrape(jam_url: Url) -> Result<Vec<Url>> {
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

    let count = game_links.len();
    cprintln!("Got %b^%B:{count}%__ games from %d^{jam_url}");
    Ok(game_links)
}
