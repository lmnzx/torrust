use serde::{Deserialize, Serialize};

use peers::Peers;

#[derive(Debug, Clone, Serialize)]
pub struct TrackerRequest {
    /// A unique identifier for your client.
    ///
    /// A string of length 20 that you get to pick.
    pub peer_id: String,

    /// The port your client is listening on.
    pub port: u16,

    /// The total amount uploaded so far.
    pub uploaded: usize,

    /// The total amount downloaded so far
    pub downloaded: usize,

    /// The number of bytes left to download.
    pub left: usize,

    /// Whether the peer list should use the compact representation
    ///
    /// The compact representation is more commonly used in the wild, the non-compact
    /// representation is mostly supported for backward-compatibility.
    pub compact: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrackerResponse {
    /// An integer, indicating how often your client should make a request to the tracker in seconds.
    ///
    /// You can ignore this value for the purposes of this challenge.
    pub interval: usize,

    /// A string, which contains list of peers that your client can connect to.
    ///
    /// Each peer is represented using 6 bytes. The first 4 bytes are the peer's IP address and the
    /// last 2 bytes are the peer's port number.
    pub peers: Peers,
}

pub fn hash_encoder(t: &[u8; 20]) -> String {
    let mut encoded = String::with_capacity(3 * t.len());
    for &byte in t {
        encoded.push('%');
        encoded.push_str(&hex::encode([byte]))
    }
    encoded
}

mod peers {
    use std::{
        fmt,
        net::{Ipv4Addr, SocketAddrV4},
    };

    use serde::{
        de::{self, Visitor},
        Deserialize, Deserializer,
    };

    #[derive(Debug, Clone)]
    pub struct Peers(pub Vec<SocketAddrV4>);

    struct PeerVisitor;

    impl<'de> Visitor<'de> for PeerVisitor {
        type Value = Peers;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a byte string with length multiple of 20")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.len() % 6 != 0 {
                return Err(E::custom("lenght is not correct"));
            }
            Ok(Peers(
                v.chunks_exact(6)
                    .map(|s| {
                        SocketAddrV4::new(
                            Ipv4Addr::new(s[0], s[1], s[2], s[3]),
                            u16::from_be_bytes([s[4], s[5]]),
                        )
                    })
                    .collect(),
            ))
        }
    }

    impl<'de> Deserialize<'de> for Peers {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de> {
            deserializer.deserialize_bytes(PeerVisitor)
        }
    }
}
