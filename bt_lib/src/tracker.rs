use crate::{
    error::{Error, Result},
    magnet::Magnet,
    peer::Peer,
};
use bendy::decoding::FromBencode;
use reqwest::Client;
use std::{borrow::Cow, collections::HashSet};
use tracing::{debug, info};
use url::Url;

const PARAMS: &[(&str, usize)] = &[
    ("port", 6881),
    ("uploaded", 0),
    ("downloaded", 0),
    ("left", 0),
    ("compact", 1),
];

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

/// All trackers
#[derive(Debug)]
pub(super) struct Trackers {
    inner: HashSet<Tracker>,
}

impl Trackers {
    pub(super) async fn new() -> Result<Self> {
        let resp = reqwest::get("https://cf.trackerslist.com/best.txt")
            .await
            .map_err(|_| Error::FetchTrackersFailed)?;
        let body = resp.text().await.unwrap();
        let inner: HashSet<_> = body
            .lines()
            .filter_map(|line| Url::parse(line).ok())
            .filter_map(Tracker::new)
            .collect();
        info!(
            "{} trackers fetched from \"https://cf.trackerslist.com/best.txt\"",
            inner.len()
        );
        Ok(Self { inner })
    }

    pub(super) async fn get_peers(
        &self,
        client: Client,
        peer_id: [u8; 20],
        magnet: &Magnet,
    ) -> Result<HashSet<Peer>> {
        async fn inner(
            client: Client,
            url: Url,
            info_hash: [u8; 20],
            peer_id: [u8; 20],
        ) -> Result<HashSet<Peer>> {
            let mut url = url;
            add_bytes_query_param(&mut url, "info_hash", &info_hash);
            add_bytes_query_param(&mut url, "peer_id", &peer_id);
            let resp = client.get(url).query(PARAMS).send().await?;
            if !resp.status().is_success() {
                return Err(Error::TrackerNoPeer);
            }
            let body = resp.bytes().await?;
            let p = Peer::from_bencode(&body).map_err(|_| Error::ParsePeerFailed)?;
            dbg!(p);
            Ok(HashSet::new())
        }
        let info_hash = magnet.hash()?;
        let mut jhs = vec![];
        if let Some(tracker) = magnet.trackers() {
            jhs.push(tokio::spawn(inner(
                client.clone(),
                tracker.url,
                info_hash,
                peer_id,
            )))
        }
        for tracker in &self.inner {
            jhs.push(tokio::spawn(inner(
                client.clone(),
                tracker.url.clone(),
                info_hash,
                peer_id,
            )))
        }
        let mut peers = HashSet::new();
        for jh in jhs {
            if let Ok(p) = jh.await {
                match p {
                    Ok(p) => peers.extend(p),
                    Err(e) => debug!("{}", e),
                }
            }
        }
        Ok(peers)
    }
}

/// Copy from [demagnetize-rs](https://github.com/jwodder/demagnetize-rs/blob/edb8aac41d337943a4dcd6eec6b56f38f7e9e51e/src/types.rs#L231)
/// and [stackoverflow](https://stackoverflow.com/questions/58026024/rust-how-to-urlencode-a-string-with-byte-parameters)
fn add_bytes_query_param(url: &mut Url, key: &str, value: &[u8]) {
    url.query_pairs_mut()
        .encoding_override(Some(&|s| {
            if s == "!" {
                Cow::Owned(value.to_vec())
            } else {
                Cow::Borrowed(s.as_bytes())
            }
        }))
        .append_pair(key, "!")
        .encoding_override(None);
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
        Trackers::new().await.unwrap();
    }
}
