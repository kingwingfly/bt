use crate::error::{Error, Result};
use std::collections::HashSet;
use url::Url;

/// The tracker
#[derive(Debug, Hash, Eq, PartialEq)]
#[non_exhaustive]
pub(super) enum Tracker {
    Http(Url),
    Udp(Url),
}

pub(super) fn trackers() -> Result<HashSet<Tracker>> {
    let resp = reqwest::blocking::get("https://cf.trackerslist.com/best.txt")
        .map_err(|_| Error::FetchTrackersFailed)?;
    let body = resp.text().unwrap();
    Ok(body
        .lines()
        .filter_map(|line| Url::parse(line).ok())
        .filter_map(|line| match line.scheme() {
            "http" => Some(Tracker::Http(line)),
            "udp" => Some(Tracker::Udp(line)),
            _ => None,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trackers() {
        let trackers = trackers();
        assert!(trackers.is_ok());
    }
}
