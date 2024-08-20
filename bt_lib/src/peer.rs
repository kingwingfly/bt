use std::net::SocketAddr;

use bendy::decoding::{FromBencode, Object};

#[derive(Debug, PartialEq, Eq, Hash)]
pub(super) struct Peer {
    addr: SocketAddr,
    peer_id: [u8; 20],
}

impl FromBencode for Peer {
    fn decode_bencode_object(object: Object<'_, '_>) -> Result<Self, bendy::decoding::Error>
    where
        Self: Sized,
    {
        let mut dict = object.try_into_dictionary()?;
        while let Some((k, v)) = dict.next_pair()? {
            match k {
                b"peers" => {
                    dbg!(v.try_into_bytes()?);
                }
                b"ip" => {
                    dbg!(v.try_into_bytes()?);
                }

                b"port" => {
                    dbg!(v.try_into_bytes()?);
                }
                _ => {}
            }
        }
        Err(bendy::decoding::Error::missing_field("todo"))
    }
}
