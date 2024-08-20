use std::time::Duration;

use crate::magnet::Magnet;
use crate::{error::Result, tracker::Trackers};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

/// The client to do network operations.
#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    trackers: Trackers,
    peer_id: [u8; 20],
}

impl Client {
    /// Create a new client.
    pub async fn new() -> Result<Self> {
        let mut rng = thread_rng();
        let mut peer_id = Vec::from(b"-rB0001-");
        while peer_id.len() < 20 {
            peer_id.push(rng.sample(Alphanumeric));
        }
        Ok(Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(8))
                .build()?,
            trackers: Trackers::new().await?,
            peer_id: peer_id.try_into().unwrap(),
        })
    }

    /// Get the torrent file.
    pub async fn get_torrent(&self, magnet: Magnet) -> Result<Vec<u8>> {
        let peers = self
            .trackers
            .get_peers(self.client.clone(), self.peer_id, &magnet)
            .await?;
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_torrent() {
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter("bt_lib=debug")
            .with_target(false)
            .without_time()
            .with_test_writer()
            .init();
        let client = Client::new().await.unwrap();
        let magnet: Magnet = "magnet:?xt=urn:btih:cfc214278888c26cb1516399a304c4f74ff6a810&dn=archlinux-2024.08.01-x86_64.iso".parse().unwrap();
        let _torrent = client.get_torrent(magnet).await.unwrap();
    }
}
