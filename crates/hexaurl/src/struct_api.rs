use std::hash;
use hexaurl_config::{validate::ValidationConfig, Config};
use serde::{Serialize, Deserialize};
use crate::{Error, encode::{encode, encode_unchecked}, decode::{decode, decode_unchecked}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct HexaUrl([u8; 16]);

impl HexaUrl {
    #[inline]
    pub fn new(input: &str, config: Option<ValidationConfig>) -> Result<Self, Error> {
        Ok(Self(encode(input, config)?))
    }

    #[inline(always)]
    pub fn new_unchecked(input: &str) -> Self {
        Self(encode_unchecked(input))
    }

    #[inline]
    pub fn decode(&self, config: Option<Config>) -> Result<String, Error> {
        decode(&self.0, config)
    }

    #[inline(always)]
    pub fn decode_unchecked(&self) -> String {
        decode_unchecked(&self.0)
    }

    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }

    #[inline(always)]
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }
}

impl hash::Hash for HexaUrl {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        let val: u128 = u128::from_le_bytes(self.0);
        val.hash(state);
    }
}

impl From<[u8; 16]> for HexaUrl {
    #[inline(always)]
    fn from(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }
}

impl AsRef<[u8; 16]> for HexaUrl {
    #[inline(always)]
    fn as_ref(&self) -> &[u8; 16] {
        self.as_bytes()
    }
}

impl AsRef<[u8]> for HexaUrl {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(feature = "ic")]
mod ic {
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl Storable for super::HexaUrl {
        fn to_bytes(&self) -> Cow<[u8]> {
            Cow::Borrowed(self.as_bytes())
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            Self::from_bytes(bytes.into_owned().try_into().unwrap())
        }

        const BOUND: Bound = Bound::Bounded {
            max_size: 16,
            is_fixed_size: true,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, BTreeMap};

    #[test]
    fn test_encode_decode() {
        let input = "HELLO";
        let hexaurl = HexaUrl::new(input, None).unwrap();
        let decoded = hexaurl.decode(None).unwrap();
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_encode_decode_unchecked() {
        let input = "HELLO";
        let hexaurl = HexaUrl::new_unchecked(input);
        let decoded = hexaurl.decode_unchecked();
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_hash() {
        let input = "HELLO";
        let hexaurl = HexaUrl::new(input, None).unwrap();
        let mut map = HashMap::new();
        map.insert(hexaurl, input);
        assert_eq!(map.get(&hexaurl), Some(&input));
    }

    #[test]
    fn test_btree_map() {
        let input = "HELLO";
        let hexaurl = HexaUrl::new(input, None).unwrap();
        let mut map = BTreeMap::new();
        map.insert(hexaurl, input);
        assert_eq!(map.get(&hexaurl), Some(&input));
    }

    #[test]
    fn test_ordering() {
        let input1 = "HELLO";
        let input2 = "WORLD";
        let hexaurl1 = HexaUrl::new(input1, None).unwrap();
        let hexaurl2 = HexaUrl::new(input2, None).unwrap();
        assert!(hexaurl1 < hexaurl2);
    }
}
