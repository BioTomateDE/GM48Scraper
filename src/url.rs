use crate::error::Context;
use reqwest::Url;

/// Get the absolute URL, relative to <https://gm48.net>.
/// The `relative_url` should not start with a slash (`/`).
pub fn get(relative_url: &str) -> crate::error::Result<Url> {
    const BASE_URL: &str = "https://gm48.net";
    let string = format!("{BASE_URL}/{relative_url}");
    let url = Url::parse(&string)?;
    Ok(url)
}

/// Extract basic game (jam) metadata from a game page url:
/// "https://gm48.net/game-jams/small-world/games/habitat" returns ("small-world, "habitat")
pub fn extract_meta(url: &Url) -> crate::error::Result<(&str, &str)> {
    let mut iter = url.as_str().rsplit("/").step_by(2).take(2);
    let game = iter.next().context("Could not extract game name")?;
    let jam = iter.next().context("Could not extract game jam name")?;
    Ok((jam, game))
}
