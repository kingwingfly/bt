use thiserror::Error;

/// Result type alias of torrent
pub type Result<T> = core::result::Result<T, Error>;

/// Error enums of torrent
#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed parsing link")]
    ParseLinkFailed(#[from] url::ParseError),
    #[error("Invalid magnet link")]
    InvalidMagnetLink,
    #[error("Failed fetching trackers")]
    FetchTrackersFailed,
    #[error("Unsupport: {}", reason)]
    Unsupport { reason: String },
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Tracker no peer")]
    TrackerNoPeer,
    #[error("Failed to parse peers")]
    ParsePeerFailed,
}
