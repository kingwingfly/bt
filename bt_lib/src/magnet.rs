use crate::{
    error::{Error, Result},
    tracker::Tracker,
};
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
    pub(super) fn hash(&self) -> Result<[u8; 20]> {
        match self.xt.strip_prefix("urn:btih:") {
            Some(data) => match data.len() {
                32 => base32::decode(base32::Alphabet::Rfc4648HexLower { padding: false }, data)
                    .ok_or(Error::InvalidMagnetLink)?
                    .try_into()
                    .map_err(|_| Error::InvalidMagnetLink),
                40 => hex::decode(data)
                    .map_err(|_| Error::InvalidMagnetLink)?
                    .try_into()
                    .map_err(|_| Error::InvalidMagnetLink),
                _ => Err(Error::Unsupport {
                    reason: "`urn:btih` only".to_string(),
                }),
            },
            None => Err(Error::Unsupport {
                reason: "`urn:btih` only".to_string(),
            }),
        }
    }

    pub(super) fn trackers(&self) -> Option<Tracker> {
        if self.tr.is_empty() {
            return None;
        }
        Url::parse(&self.tr).ok().and_then(Tracker::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnet_parser() {
        let s = "magnet:?xt=urn:btih:cfc214278888c26cb1516399a304c4f74ff6a810&dn=archlinux-2024.08.01-x86_64.iso";
        let magnet = s.parse::<Magnet>().unwrap();
        let _hash = magnet.hash().unwrap();
    }
}
