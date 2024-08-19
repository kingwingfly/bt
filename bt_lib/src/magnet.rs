use crate::error::{Error, Result};
use std::str::FromStr;
use url::Url;

/// Magnet link
#[derive(Debug, Default, Clone)]
pub struct Magnet {
    /// exact topic
    xt: String,
    /// display name
    dn: String,
    /// tracker
    tr: String,
}

impl FromStr for Magnet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let url = Url::parse(s)?;
        if url.scheme() != "magnet" {
            return Err(Error::InvalidMagnetLink);
        }
        let mut magnet = Magnet::default();
        for (k, v) in url.query_pairs() {
            match k.as_ref() {
                "xt" => magnet.xt = v.into_owned(),
                "dn" => magnet.dn = v.into_owned(),
                "tr" => magnet.tr = v.into_owned(),
                _ => {}
            }
        }
        Ok(magnet)
    }
}

impl Magnet {
    pub(super) fn urn(&self) -> &str {
        &self.xt[4..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnet_parser() {
        let s = "magnet:?xt=urn:btih:cfc214278888c26cb1516399a304c4f74ff6a810&dn=archlinux-2024.08.01-x86_64.iso";
        assert!(s.parse::<Magnet>().is_ok());
    }
}
