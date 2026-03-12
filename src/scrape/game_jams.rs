use crate::error::{Result, bail};
use crate::html::{extract_href, get_html};
use crate::url::get_url;
use reqwest::Url;
use scraper::Selector;
use std::sync::LazyLock;

static SELECTOR: LazyLock<Selector> = LazyLock::new(make_selector);

fn make_selector() -> Selector {
    Selector::parse("#jamModal .modal-body > .list-group > a").unwrap()
}

pub async fn scrape() -> Result<Vec<Url>> {
    let url = get_url("game-jams/top-down/games")?;
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
