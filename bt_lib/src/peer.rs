use bendy::decoding::{FromBencode, Object};
use std::{collections::HashSet, net::SocketAddr, ops};
use tracing::trace;

#[derive(Debug, PartialEq, Eq, Hash)]
pub(super) struct Peer {
    addr: SocketAddr,
}

#[derive(Debug, Default)]
pub(super) struct Peers(HashSet<Peer>);

impl Peers {
    pub(super) fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromBencode for Peers {
    fn decode_bencode_object(object: Object<'_, '_>) -> Result<Self, bendy::decoding::Error>
    where
        Self: Sized,
    {
        let mut dict = object.try_into_dictionary()?;
        let mut hs = HashSet::new();
        while let Some((k, v)) = dict.next_pair()? {
            if k == b"peers" {
                let bytes = v.try_into_bytes()?;
                for i in (0..bytes.len()).step_by(6) {
                    let ip = format!(
                        "{}.{}.{}.{}",
                        bytes[i],
                        bytes[i + 1],
                        bytes[i + 2],
                        bytes[i + 3]
                    );
                    let port = u16::from_be_bytes([bytes[i + 4], bytes[i + 5]]);
                    if let Ok(ip) = ip.parse() {
                        let addr = SocketAddr::new(ip, port);
                        trace!("fetch peer: {}", addr);
                        hs.insert(Peer { addr });
                    }
                }
            }
        }
        Ok(Self(hs))
    }
}

impl ops::BitOrAssign for Peers {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.extend(rhs.0);
    }
}
