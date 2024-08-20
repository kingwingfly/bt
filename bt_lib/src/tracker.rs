use crate::error::{Error, Result};
use std::collections::HashSet;
use tracing::info;
use url::Url;

/// The tracker
#[derive(Debug, Hash, Eq, PartialEq)]
pub(super) struct Tracker {
    pub(super) url: Url,
}

impl Tracker {
    pub(super) fn new(url: Url) -> Option<Self> {
        match url.scheme() {
            "http" => Some(Self { url }),
            _ => None,
        }
    }
}

pub(super) async fn trackers() -> Result<HashSet<Tracker>> {
    let resp = reqwest::get("https://cf.trackerslist.com/best.txt")
        .await
        .map_err(|_| Error::FetchTrackersFailed)?;
    let body = resp.text().await.unwrap();
    let trackers: HashSet<_> = body
        .lines()
        .filter_map(|line| Url::parse(line).ok())
        .filter_map(Tracker::new)
        .collect();
    info!(
        "{} trackers fetched from \"https://cf.trackerslist.com/best.txt\"",
        trackers.len()
    );
    Ok(trackers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_trackers() {
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter("bt_lib=debug")
            .with_target(false)
            .without_time()
            .with_test_writer()
            .init();
        let trackers = trackers().await;
        assert!(trackers.is_ok());
    }
}
