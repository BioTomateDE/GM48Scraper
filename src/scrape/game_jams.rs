use crate::error::{Result, bail};
use crate::{html, url};
use reqwest::Url;
use scraper::Selector;

pub async fn scrape() -> Result<Vec<Url>> {
    let url = url::get("game-jams/top-down/games")?;
    let html = html::get(url).await?;

    let selector = "#jamModal .modal-body > .list-group > a";
    let selector = Selector::parse(selector).unwrap();
    let mut game_jam_links = Vec::new();

    for element in html.select(&selector) {
        let href = html::extract::href(element)?;
        game_jam_links.push(href);
    }

    if game_jam_links.is_empty() {
        bail!("Could not find any Game Jams in HTML");
    }

    Ok(game_jam_links)
}
