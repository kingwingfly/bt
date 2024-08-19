use thiserror::Error;

/// Result type alias of torrent
pub type Result<T> = core::result::Result<T, Error>;

/// Error enums of torrent
#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed parsing link")]
    ParseLinkError(#[from] url::ParseError),
    #[error("Invalid magnet link")]
    InvalidMagnetLinkError,
}
