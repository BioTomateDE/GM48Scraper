use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use reqwest::Url;
use std::sync::LazyLock;

const BASE_URL_RAW: &str = "https://gm48.net";
pub static BASE_URL: LazyLock<Url> = LazyLock::new(|| Url::parse(BASE_URL_RAW).unwrap());

/// Extract basic game (jam) metadata from a game page url:
/// <https://gm48.net/game-jams/small-world/games/habitat> returns ("small-world", "habitat")
pub fn extract_meta(url: &Url) -> Result<(&str, &str)> {
    let mut segments = url.path_segments().context("Cannot-be-a-base URL")?;
    let segment_count = segments.clone().count();
    if segment_count != 4 {
        bail!("Expected URL to have 4 path segments; actually has {segment_count}: {url}");
    }

    let first = segments.next().unwrap();
    if first != "game-jams" {
        bail!("Expected first URL path segment to be `game-jams`; got {first:?}");
    }

    let jam = segments.next().unwrap();

    let third = segments.next().unwrap();
    if third != "games" {
        bail!("Expected third URL path segment to be `games`; got {third:?}");
    }

    let game = segments.next().unwrap();
    Ok((jam, game))
}

mod tests {
    #[test]
    fn parse_static_base_url() {
        _ = &*super::BASE_URL;
    }
}
