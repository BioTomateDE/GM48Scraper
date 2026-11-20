use crate::html::extract_href;
use crate::html::get_html;
use anyhow::Result;
use colored_print::cprintln;
use reqwest::Url;
use scraper::Selector;
use std::sync::LazyLock;

const SELECTOR_RAW: &str = "#games .single-game > a";
static SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse(SELECTOR_RAW).unwrap());

pub async fn scrape_games(jam_url: Url) -> Result<Vec<Url>> {
    let html = get_html(jam_url.clone()).await?;

    let mut game_links = Vec::new();

    for element in html.select(&SELECTOR) {
        let href = extract_href(element)?;
        game_links.push(href);
    }

    // This game list is allowed to be empty:
    // There could potentially be an ongoing game jam with no results yet.

    let count = game_links.len();
    cprintln!("Got %b^%B:{count}%__ games from %d^{jam_url}");
    Ok(game_links)
}

mod tests {
    #[test]
    fn selector_is_valid() {
        let _ = &*super::SELECTOR;
    }
}
