use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

use pieces::Pieces;

/// Metainfo file / torrent file
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Torrent {
    /// The URL of the tracker
    pub announce: String,

    pub info: Info,
}

impl Torrent {
    pub fn info_hash(&self) -> [u8; 20] {
        let info_encoded = serde_bencode::to_bytes(&self.info).expect("re-encode info section");
        let mut hasher = Sha1::new();
        hasher.update(&info_encoded);
        hasher
            .finalize()
            .try_into()
            .expect("GenericArray<_, 20> ==  [_;20]")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Info {
    /// The suggested name to save the file (or directory) as. It is purely advisory.
    pub name: String,

    /// The number of bytes in each piece the file is split into.
    #[serde(rename = "piece length")]
    pub plength: usize,

    /// Each entry of `pieces` is the SHA1 hash of the piece at the corresponding index.
    pub pieces: Pieces,

    #[serde(flatten)]
    pub keys: Keys,
}

/// There is a key `length` or a key `files`, but not both or neither.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Keys {
    /// If `length` is present then the download represents a single file.
    SingleFile {
        /// The length of the file in bytes.
        length: usize,
    },
    /// Otherwise it represents a set of files which go in a directory structure.
    ///
    /// For the purposes of the other keys in `Info`, the multi-file case is treated as only having
    /// a single file by concatenating the files in the order they appear in the files list.
    MultiFile { files: Vec<File> },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
    /// The length of the file, in bytes.
    pub length: usize,

    /// Subdirectory names for this file, the last of which is the actual file name
    pub path: Vec<String>,
}

mod pieces {
    use serde::{
        de::{self, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };
    use std::fmt;

    #[derive(Debug, Clone)]
    pub struct Pieces(pub Vec<[u8; 20]>);

    struct PiecesVisitor;

    impl<'de> Visitor<'de> for PiecesVisitor {
        type Value = Pieces;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a byte string with length multiple of 20")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.len() % 20 != 0 {
                return Err(E::custom("length is not correct"));
            }
            Ok(Pieces(
                v.chunks_exact(20)
                    .map(|s| s.try_into().expect("length is 20"))
                    .collect(),
            ))
        }
    }

    impl<'de> Deserialize<'de> for Pieces {
       fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
           where
               D: Deserializer<'de> {
           deserializer.deserialize_bytes(PiecesVisitor)
       } 
    }

    impl Serialize for Pieces {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer {
            let slice = self.0.concat();
            serializer.serialize_bytes(&slice)
        }
    }
}
