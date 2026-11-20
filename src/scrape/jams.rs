use crate::html::extract_href;
use crate::html::get_html;
use crate::url::BASE_URL;
use anyhow::Result;
use anyhow::bail;
use reqwest::Url;
use scraper::Selector;
use std::sync::LazyLock;

const SELECTOR_RAW: &str = "#jamModal .modal-body > .list-group > a";
static SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse(SELECTOR_RAW).unwrap());

pub async fn scrape() -> Result<Vec<Url>> {
    let url = BASE_URL.join("game-jams/top-down/games")?;
    let html = get_html(url).await?;

    let mut game_jam_links = Vec::new();

    for element in html.select(&SELECTOR) {
        let href = extract_href(element)?;
        game_jam_links.push(href);
    }

    if game_jam_links.is_empty() {
        bail!("Could not find any Game Jams in HTML");
    }

    Ok(game_jam_links)
}

mod tests {
    #[test]
    fn selector_is_valid() {
        let _ = &*super::SELECTOR;
    }
}
