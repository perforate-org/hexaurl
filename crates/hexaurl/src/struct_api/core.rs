use std::fmt;
use hexaurl_config::Config;
use crate::{
    Error,
    decode::{decode, decode_with_config, decode_unchecked, decode_core},
    encode::{encode, encode_with_config, encode_minimal_checked, encode_unchecked},
    validate::validate_with_config,
};

/// A wrapper around a fixed byte array representing a HexaURL.
///
/// HexaUrl provides methods to encode a string into a fixed byte representation
/// and decode it back to the original string. It supports various encoding
/// options including minimal checking and unchecked encoding.
///
/// # Generic Parameters
/// - `N`: The length of the byte array.
/// - `S`: The maximum length of the encoded HexaURL string.
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HexaUrl<const N: usize = 16, const S: usize = 21>([u8; N]);

impl<const N: usize, const S: usize> HexaUrl<N, S> {
    /// Encodes the input string and creates a new `HexaUrl` with the default validation config.
    ///
    /// # Arguments
    ///
    /// * `input` - The string to encode.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the encoding fails.
    #[inline]
    pub fn new(input: &str) -> Result<Self, Error> {
        Ok(Self(encode(input)?))
    }

    /// Encodes the input string using a specific validation configuration and
    /// creates a new `HexaUrl`.
    ///
    /// # Arguments
    ///
    /// * `input` - The string to encode.
    /// * `config` - The configuration for validation.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the encoding with configuration fails.
    #[inline]
    pub fn new_with_config(input: &str, config: Config) -> Result<Self, Error> {
        Ok(Self(encode_with_config(input, config)?))
    }

    /// Encodes the input string with minimal checks and creates a new `HexaUrl`.
    ///
    /// Returns `None` if the minimal checked encoding fails.
    ///
    /// # Arguments
    ///
    /// * `input` - The string to encode.
    #[inline(always)]
    pub fn new_minimal_checked(input: &str) -> Option<Self> {
        Some(Self(encode_minimal_checked(input)?))
    }

    /// Encodes the input string without any checks and creates a new `HexaUrl`.
    ///
    /// # Arguments
    ///
    /// * `input` - The string to encode.
    #[inline(always)]
    pub fn new_unchecked(input: &str) -> Self {
        Self(encode_unchecked(input))
    }

    #[inline]
    pub fn decode(&self) -> Result<String, Error> {
        decode::<N, S>(&self.0)
    }

    /// Decodes the HexaUrl into the original string using an optional configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Optional configuration to adjust the decode process.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if decoding fails.
    #[inline]
    pub fn decode_with_config(&self, config: Config) -> Result<String, Error> {
        decode_with_config::<N, S>(&self.0, config)
    }

    #[inline(always)]
    pub fn decode_minimal_checked(&self) -> Result<String, Error> {
        decode_with_config::<N, S>(&self.0, Config::minimal())
    }

    /// Decodes the HexaUrl into the original string without performing any checks.
    #[inline(always)]
    pub fn decode_unchecked(&self) -> String {
        decode_unchecked::<N, S>(&self.0)
    }

    /// Returns the underlying byte array.
    #[inline(always)]
    pub const fn as_slice(&self) -> &[u8; N] {
        &self.0
    }

    #[inline]
    pub fn try_from_slice(bytes: &[u8; N]) -> Result<Self, Error> {
        let mut dst = [0; S];
        let str = unsafe { std::str::from_utf8_unchecked(decode_core(bytes, &mut dst)) };
        validate_with_config::<N>(str, Config::minimal())?;

        let mut arr = [0; N];
        arr.copy_from_slice(bytes);
        Ok(Self(arr))
    }

    /// Returns the maximum length of the encoded HexaUrl.
    #[inline]
    pub const fn max_len() -> usize {
        S
    }
}

impl<const N: usize, const S: usize> fmt::Display for HexaUrl<N, S> {
    /// Formats the HexaUrl as a string.
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let decoded = self.decode_minimal_checked().map_err(|_| fmt::Error)?;
        write!(f, "{}", decoded)
    }
}

impl<const N: usize> TryFrom<&str> for HexaUrl<N> {
    type Error = Error;

    /// Tries to create a `HexaUrl` from a string.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the string is not valid.
    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<const N: usize> TryFrom<String> for HexaUrl<N> {
    type Error = Error;

    /// Tries to create a `HexaUrl` from a string.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the string is not valid.
    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl<const N: usize> TryFrom<&String> for HexaUrl<N> {
    type Error = Error;

    /// Tries to create a `HexaUrl` from a string.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the string is not valid.
    #[inline]
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<const N: usize, const S: usize> TryFrom<&[u8]> for HexaUrl<N, S> {
    type Error = Error;

    /// Tries to create a `HexaUrl` from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the byte slice is not valid.
    #[inline]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != N {
            return Err(Error::InvalidLength);
        }
        let mut bytes = [0; N];
        bytes.copy_from_slice(value);

        Self::try_from_slice(&bytes)
    }
}

impl<const N: usize, const S: usize> TryFrom<[u8; N]> for HexaUrl<N, S> {
    type Error = Error;

    /// Converts a byte array into a `HexaUrl`.
    #[inline(always)]
    fn try_from(bytes: [u8; N]) -> Result<Self, Self::Error> {
        Self::try_from_slice(&bytes)
    }
}

impl<const N: usize, const S: usize> TryFrom<&[u8; N]> for HexaUrl<N, S> {
    type Error = Error;

    /// Converts a byte array into a `HexaUrl`.
    #[inline(always)]
    fn try_from(bytes: &[u8; N]) -> Result<Self, Self::Error> {
        Self::try_from_slice(bytes)
    }
}

impl<const N: usize> AsRef<[u8; N]> for HexaUrl<N> {
    /// Provides a reference to the underlying byte array.
    #[inline(always)]
    fn as_ref(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize> AsRef<[u8]> for HexaUrl<N> {
    /// Provides a reference to the underlying byte slice.
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    impl<const N: usize> serde::Serialize for HexaUrl<N> {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            if serializer.is_human_readable() {
                self.to_string().serialize(serializer)
            } else {
                serializer.serialize_bytes(self.as_slice())
            }
        }
    }

    mod deserialize {
        use super::HexaUrl;
        use std::convert::TryFrom;

        pub(super) struct HexaUrlVisitor<const N:usize>;

        #[allow(clippy::needless_lifetimes)]
        impl<'de, const N: usize> serde::de::Visitor<'de> for HexaUrlVisitor<N> {
            type Value = HexaUrl<N>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("bytes or string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                HexaUrl::new(value).map_err(E::custom)
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                HexaUrl::try_from(value).map_err(E::custom)
            }
        }
    }

    impl<'de, const N: usize> serde::Deserialize<'de> for HexaUrl<N> {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<HexaUrl<N>, D::Error> {
            use serde::de::Error;
            if deserializer.is_human_readable() {
                deserializer
                    .deserialize_str(deserialize::HexaUrlVisitor)
                    .map_err(D::Error::custom)
            } else {
                deserializer
                    .deserialize_bytes(deserialize::HexaUrlVisitor)
                    .map_err(D::Error::custom)
            }
        }
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, const N: usize, const S: usize> arbitrary::Arbitrary<'a> for HexaUrl<N, S> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        use crate::{decode::decode_core, validate::validate_with_config};

        let len = u.int_in_range(0..=N)?;
        let mut bytes = [0; N];
        u.fill_buffer(&mut bytes[..len])?;

        let mut dst = [0; S];
        let str = unsafe { std::str::from_utf8_unchecked(decode_core(&bytes, &mut dst)) };
        validate_with_config::<N>(str, Config::minimal()).map_err(|_| arbitrary::Error::IncorrectFormat)?;

        Ok(Self(bytes))
    }
}

#[cfg(feature = "ic")]
mod ic {
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    /// Implements the `Storable` trait for `HexaUrl` for use with Internet Computer stable structures.
    impl<const N: usize> Storable for super::HexaUrl<N> {
        fn to_bytes(&self) -> Cow<[u8]> {
            Cow::Borrowed(&self.0[..])
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            assert_eq!(bytes.len(), N);
            let mut arr = [0; N];
            arr[0..N].copy_from_slice(&bytes);
            Self(arr)
        }

        const BOUND: Bound = Bound::Bounded {
            max_size: N as u32,
            is_fixed_size: true,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, BTreeMap};
    use serde_json;

    /// Tests encoding and decoding of a string using the default configuration.
    #[test]
    fn test_encode_decode() {
        let input = "hello";
        let hexaurl = HexaUrl::<16>::new(input).unwrap();
        let decoded = hexaurl.decode().unwrap();
        assert_eq!(input, decoded);
    }

    /// Tests the unchecked encoding and decoding of a string.
    #[test]
    fn test_encode_decode_unchecked() {
        let input = "hello";
        let hexaurl = HexaUrl::<16>::new_unchecked(input);
        let decoded = hexaurl.decode_unchecked();
        assert_eq!(input, decoded);
    }

    /// Tests that `HexaUrl` implements the Hash trait properly by using it as a key in a HashMap.
    #[test]
    fn test_hash() {
        let input = "hello";
        let hexaurl = HexaUrl::<16>::new(input).unwrap();
        let mut map = HashMap::new();
        map.insert(hexaurl, input);
        assert_eq!(map.get(&hexaurl), Some(&input));
    }

    /// Tests that `HexaUrl` implements ordering correctly by using it as a key in a BTreeMap.
    #[test]
    fn test_btree_map() {
        let input = "hello";
        let hexaurl = HexaUrl::<16>::new(input).unwrap();
        let mut map = BTreeMap::new();
        map.insert(hexaurl, input);
        assert_eq!(map.get(&hexaurl), Some(&input));
    }

    /// Tests the ordering between two `HexaUrl` values created from different strings.
    #[test]
    fn test_ordering() {
        let input1 = "hello";
        let input2 = "world";
        let hexaurl1 = HexaUrl::<16>::new(input1).unwrap();
        let hexaurl2 = HexaUrl::<16>::new(input2).unwrap();
        assert!(hexaurl1 < hexaurl2);
    }

    /// Tests successful creation of HexaUrl from a byte slice.
    #[test]
    fn test_try_from_bytes_success() {
        let input = "hello";
        let hexaurl = HexaUrl::<16>::new(input).unwrap();
        let bytes = hexaurl.as_slice();
        let hexaurl_copy = HexaUrl::<16>::try_from(&bytes[..]).unwrap();
        assert_eq!(hexaurl, hexaurl_copy);
    }

    /// Tests that trying to create a HexaUrl from a byte slice with invalid length returns an error.
    #[test]
    fn test_try_from_bytes_invalid_length() {
        let bytes = [0u8; 15]; // Incorrect length
        let result = HexaUrl::<16>::try_from(&bytes[..]);
        assert!(result.is_err());
    }

    /// Tests encoding and decoding using a specific configuration.
    #[test]
    fn test_new_with_config() {
        let input = "hello";
        let config = Config::minimal();
        let hexaurl = HexaUrl::<16>::new_with_config(input, config).unwrap();
        let decoded = hexaurl.decode_with_config(config).unwrap();
        assert_eq!(input, decoded);
    }

    #[cfg(feature = "serde")]
    mod serde_impl {
        use super::*;

        /// Tests serialization and deserialization in a human-readable format.
        #[test]
        fn test_serde_serialization_human_readable() {
            let input = "hello";
            let hexaurl = HexaUrl::<16>::new(input).unwrap();
            let json = serde_json::to_string(&hexaurl).unwrap();
            let deserialized: HexaUrl<16> = serde_json::from_str(&json).unwrap();
            assert_eq!(hexaurl, deserialized);
        }

        /// Tests serialization in a non-human-readable context.
        #[test]
        fn test_serde_serialization_non_human_readable() {
            // Note: serde_json is always human-readable, so we simulate a non-human-readable serializer using bincode if available.
            // Here we only check that the process does not panic and round-trips correctly.
            let input = "hello";
            let hexaurl = HexaUrl::<16>::new(input).unwrap();
            let encoded = bincode::serialize(&hexaurl).unwrap();
            let decoded: HexaUrl<16> = bincode::deserialize(&encoded).unwrap();
            assert_eq!(hexaurl, decoded);
        }
    }

    #[cfg(feature = "arbitrary")]
    mod arbitrary_impl {
        use super::*;
        use arbitrary::Arbitrary;
        use arbtest::arbtest;

        /// Tests that `HexaUrl` implements the `Arbitrary` trait correctly.
        #[test]
        fn test_arbitrary() {
            fn prop(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<()> {
                let hexaurl = HexaUrl::<16>::arbitrary(u)?;
                let decoded = hexaurl.decode_with_config(Config::minimal()).unwrap();
                assert_eq!(hexaurl.to_string(), decoded);
                Ok(())
            }
            arbtest(prop).budget_ms(1_000).run();
        }
    }
}
