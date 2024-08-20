use std::borrow::Cow;

use crate::error::Result;
use crate::tracker::trackers;
use crate::Magnet;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use url::Url;

const PARAMS: &[(&str, usize)] = &[
    ("port", 6881),
    ("uploaded", 0),
    ("downloaded", 0),
    ("left", 0),
    ("compact", 1),
];

/// The client to do network operations.
#[derive(Debug, Default)]
pub struct Client {
    client: reqwest::Client,
    peer_id: [u8; 20],
}

impl Client {
    /// Create a new client.
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut peer_id = Vec::from(b"-rB0001-");
        while peer_id.len() < 20 {
            peer_id.push(rng.sample(Alphanumeric));
        }
        Self {
            client: reqwest::Client::new(),
            peer_id: peer_id.try_into().unwrap(),
        }
    }

    /// Get the torrent file.
    pub async fn get_torrent(&self, magnet: Magnet) -> Result<Vec<u8>> {
        let hash = magnet.hash()?;
        // fetch trackers
        let mut trackers = trackers().await?;
        if let Some(tracker) = magnet.trackers() {
            trackers.insert(tracker);
        }
        // fetch peers
        for tracker in trackers {
            let mut url = tracker.url.clone();
            add_bytes_query_param(&mut url, "info_hash", &hash);
            add_bytes_query_param(&mut url, "peer_id", &self.peer_id);
            let resp = self.client.get(url).query(PARAMS).send().await?;
            dbg!(resp.status());
            let body = resp.bytes().await?;

            dbg!(body);
        }
        // ...
        Ok(vec![])
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
    async fn test_get_torrent() {
        let client = Client::new();
        let magnet: Magnet = "magnet:?xt=urn:btih:cfc214278888c26cb1516399a304c4f74ff6a810&dn=archlinux-2024.08.01-x86_64.iso".parse().unwrap();
        let _torrent = client.get_torrent(magnet).await.unwrap();
    }
}
