#[allow(unused_imports)]
use super::{HexaUrl256, HexaUrl8};
use crate::{
    decode::{decode, decode_core, decode_unchecked, decode_with_config},
    encode::{encode, encode_minimal_config, encode_quick, encode_unchecked, encode_with_config},
    validate::validate_minimal_config,
    Error,
    utils::len,
    MASK_TWO_BITS,
    MASK_FOUR_BITS,
    MASK_SIX_BITS,
};
use hexaurl_config::Config;
use std::{fmt::{self, Binary}, str};

/// A wrapper around a fixed-size byte array representing a HexaURL.
///
/// ---
///
/// **Note:** This structure is typically accessed via the public type aliases [`HexaUrl8`] through [`HexaUrl256`],
/// which correspond to internal types `HexaUrlCore<8, 10>` through `HexaUrlCore<256, 341>`. Due to current compile-time
/// limitations—in which the encoded string length (S) is derived from the byte array size (N) using the formula S = N * 4 / 3—
/// only predefined, valid (N, S) pairs are supported[^note].
///
/// The `HexaUrlCore` type can be made directly public with the `pub-struct-core` feature.
///
/// [^note]: The compiler currently cannot enforce this constraint, but once the [generic_const_exprs](https://github.com/rust-lang/rust/issues/76560) feature becomes stable, this limitation is expected to be relaxed.
///
/// ---
///
/// `HexaUrlCore` provides methods to encode strings into fixed-size byte representations and to decode them back
/// into their case-insensitive original strings. It supports several encoding options:
/// - Standard encoding with full validation
/// - Custom validation with user-specified configuration
/// - Quick validation for improved performance
/// - Unchecked encoding for trusted input
///
/// The encoded bytes have a fixed size determined by the generic parameters.
///
/// # Generic Parameters
///
/// - `N`: The size of the internal byte array storage.
/// - `S`: The maximum length of the encoded HexaURL string representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HexaUrlCore<const N: usize, const S: usize>([u8; N]);

impl<const N: usize, const S: usize> HexaUrlCore<N, S> {
    /// Encodes the input string using the default validation rules and creates a new `HexaUrlCore`.
    ///
    /// This is the recommended method for encoding when full validation is desired.
    ///
    /// # Arguments
    ///
    /// - `input` - The string to be encoded.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if:
    /// - The input string does not satisfy the default validation rules.
    /// - The encoded result exceeds the fixed size limits.
    #[inline]
    pub fn new(input: &str) -> Result<Self, Error> {
        Ok(Self(encode(input)?))
    }

    /// Encodes the input string using a custom validation configuration and creates a new `HexaUrlCore`.
    ///
    /// Use this method when fine-grained control over validation is needed.
    ///
    /// # Arguments
    ///
    /// - `input` - The string to be encoded.
    /// - `config` - The custom validation configuration to use.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if:
    /// - The input fails validation according to the provided configuration.
    /// - The encoded result exceeds the fixed size limits.
    #[inline]
    pub fn new_with_config(input: &str, config: Config) -> Result<Self, Error> {
        Ok(Self(encode_with_config(input, config)?))
    }

    /// Encodes the input string with minimal validation and creates a new `HexaUrlCore`.
    ///
    /// This method uses the minimal validation rules provided by [`Config::minimal()`].
    ///
    /// # Arguments
    ///
    /// - `input` - The string to be encoded with minimal validation.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if:
    /// - The input does not pass the minimal validation.
    /// - The encoded result exceeds the fixed size limits.
    #[inline]
    pub fn new_minimal_config(input: &str) -> Result<Self, Error> {
        Ok(Self(encode_minimal_config(input)?))
    }

    /// Encodes the input string using quick validation checks and creates a new `HexaUrlCore`.
    ///
    /// This method provides better performance than full validation at the cost of reduced safety.
    /// Use it only when temporary acceptance of potentially invalid encoding is acceptable.
    ///
    /// # Arguments
    ///
    /// - `input` - The string to be encoded.
    ///
    /// # Returns
    ///
    /// Returns a `HexaUrlCore` wrapped in `Result` if the quick validation checks pass.
    #[inline(always)]
    pub fn new_quick(input: &str) -> Result<Self, Error> {
        Ok(Self(encode_quick(input)?))
    }

    /// Encodes the input string without any validation and creates a new `HexaUrlCore`.
    ///
    /// # Safety
    ///
    /// <div class="warning">The input string must be ASCII. Otherwise, it causes undefined behavior.</div>
    ///
    /// # Arguments
    ///
    /// - `input` - The string to be encoded without validation.
    #[inline(always)]
    pub unsafe fn new_unchecked(input: &str) -> Self {
        Self(unsafe { encode_unchecked(input) })
    }

    /// Decodes the `HexaUrlCore` back into a `String` using the default validation rules.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the decoded string fails the validation checks.
    #[inline]
    pub fn decode(&self) -> Result<String, Error> {
        decode::<N, S>(&self.0)
    }

    /// Decodes the `HexaUrlCore` into a `String` using a custom validation configuration.
    ///
    /// # Arguments
    ///
    /// - `config` - The custom validation configuration to apply during decoding.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if:
    /// - Decoding fails according to the provided configuration.
    /// - The decoded string is not valid UTF-8.
    #[inline]
    pub fn decode_with_config(&self, config: Config) -> Result<String, Error> {
        decode_with_config::<N, S>(&self.0, config)
    }

    /// Decodes the `HexaUrlCore` into a `String` without performing any validation.
    ///
    /// # Safety
    ///
    /// This method is unsafe because it bypasses validation. It should only be used with trusted data.
    #[inline(always)]
    pub fn decode_unchecked(&self) -> String {
        decode_unchecked::<N, S>(&self.0)
    }

    /// Returns a reference to the underlying byte array.
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8; N] {
        &self.0
    }

    /// Attempts to create a `HexaUrlCore` from a raw byte slice.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if:
    /// - The bytes do not pass minimal validation.
    /// - The decoded string is not valid UTF-8.
    #[inline]
    pub fn try_from_bytes(bytes: &[u8; N]) -> Result<Self, Error> {
        let mut dst = [0; S];
        let str = unsafe { str::from_utf8_unchecked(decode_core(bytes, &mut dst)) };
        validate_minimal_config::<N>(str)?;

        let mut arr = [0; N];
        arr.copy_from_slice(bytes);
        Ok(Self(arr))
    }

    /// Creates a new `HexaUrlCore` from a byte slice without any validation or bounds checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure that bytes parameter is valid and correctly sized for the target type.
    /// No validation is performed.
    #[inline]
    pub const unsafe fn from_slice(bytes: &[u8; N]) -> Self {
        Self(*bytes)
    }

    /// Returns the maximum possible length of the encoded `HexaUrlCore` string.
    #[inline(always)]
    pub const fn capacity() -> usize {
        S
    }

    /// Returns the length of the encoded string representation.
    ///
    /// O(log N)
    #[inline(always)]
    pub fn len(&self) -> usize {
        let byte_len = self.byte_len();
        if byte_len == 0 {
            return 0;
        }

        // Calculate base length from full 3-byte chunks
        let base_len = byte_len / 3 * 4;

        // Handle remaining bytes and trailing zeros
        let remainder = byte_len % 3;
        let last_byte = self.0[byte_len - 1];

        let len = match remainder {
            0 => {
                if (last_byte & MASK_SIX_BITS) == 0 {
                    base_len - 1
                } else {
                    base_len
                }
            },
            1 => {
                if (last_byte & MASK_TWO_BITS) == 0 {
                    base_len + 1
                } else {
                    base_len + 2
                }
            }
            2 => {
                if (last_byte & MASK_FOUR_BITS) == 0 {
                    base_len + 2
                } else {
                    base_len + 3
                }
            }
            _ => unreachable!(),
        };

        if len > S {
            S
        } else {
            len
        }
    }

    /// Returns the length of the byte representation.
    #[inline(always)]
    fn byte_len(&self) -> usize {
        len(&self.0)
    }

    /// Returns true if the encoded string representation is empty.
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.0[0] == 0
    }
}

impl<const M: usize, const T: usize> HexaUrlCore<M, T> {
    /// Converts an HexaUrlCore\<M\> to an HexaUrlCore\<N\>. If the length of the bytes being
    /// converted is greater than N, the extra characters are ignored.
    /// This operation produces a copy (non-destructive).
    ///
    /// # Example
    ///
    ///```ignore
    ///  let s1: HexaUrlCore<8> = HexaUrlCore::new("abcdefg")?;
    ///  let s2: HexaUrlCore<16> = s1.resize();
    ///```
    pub fn resize<const N: usize, const S: usize>(&self) -> HexaUrlCore<N, S> {
        let byte_len = self.byte_len();
        self.resize_core(byte_len)
    }

    /// Version of resize that does not allow string truncation due to length.
    pub fn reallocate<const N: usize, const S: usize>(&self) -> Option<HexaUrlCore<N, S>> {
        let byte_len = self.byte_len();
        if byte_len < N {
            Some(self.resize_core(byte_len))
        } else {
            None
        }
    }

    fn resize_core<const N: usize, const S: usize>(&self, byte_len: usize) -> HexaUrlCore<N, S> {
        let length = if byte_len < N {
            byte_len
        } else {
            N
        };
        let mut arr = [0; N];
        arr[..length].copy_from_slice(&self.0[..length]);
        HexaUrlCore(arr)
    }
}

impl<const N: usize, const S: usize> fmt::Display for HexaUrlCore<N, S> {
    /// Formats the `HexaUrlCore` as its decoded string representation.
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res: [u8; S] = [0; S];
        let slice = decode_core::<N, S>(&self.0, &mut res);
        // SAFETY: The function assumes the input is valid and does not contain any null bytes.
        let str_inner = unsafe { str::from_utf8_unchecked(slice) };
        f.pad(str_inner)
    }
}

impl<const N: usize, const S: usize> From<HexaUrlCore<N, S>> for String {
    /// Converts the `HexaUrlCore` into its decoded string representation.
    #[inline]
    fn from(value: HexaUrlCore<N, S>) -> String {
        value.to_string()
    }
}

impl<const N: usize, const S: usize> TryFrom<String> for HexaUrlCore<N, S> {
    type Error = Error;

    /// Attempts to create a `HexaUrlCore` from a String.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if validation fails or conversion is impossible.
    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl<const N: usize, const S: usize> TryFrom<&String> for HexaUrlCore<N, S> {
    type Error = Error;

    /// Attempts to create a `HexaUrlCore` from a String reference.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if validation fails or conversion is impossible.
    #[inline]
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::new_minimal_config(value)
    }
}

impl<const N: usize, const S: usize> TryFrom<&[u8]> for HexaUrlCore<N, S> {
    type Error = Error;

    /// Attempts to create a `HexaUrlCore` from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if:
    /// - The slice length doesn't match N
    /// - The bytes fail validation
    #[inline]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != N {
            return Err(Error::InvalidLength);
        }
        let mut bytes = [0; N];
        bytes.copy_from_slice(value);

        Self::try_from_bytes(&bytes)
    }
}

impl<const N: usize, const S: usize> TryFrom<[u8; N]> for HexaUrlCore<N, S> {
    type Error = Error;

    /// Attempts to create a `HexaUrlCore` from a fixed-size byte array.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the bytes fail validation.
    #[inline(always)]
    fn try_from(bytes: [u8; N]) -> Result<Self, Self::Error> {
        Self::try_from_bytes(&bytes)
    }
}

impl<const N: usize, const S: usize> TryFrom<&[u8; N]> for HexaUrlCore<N, S> {
    type Error = Error;

    /// Attempts to create a `HexaUrlCore` from a reference to a fixed-size byte array.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the bytes fail validation.
    #[inline(always)]
    fn try_from(bytes: &[u8; N]) -> Result<Self, Self::Error> {
        Self::try_from_bytes(bytes)
    }
}

impl<const N: usize, const S: usize> AsRef<[u8; N]> for HexaUrlCore<N, S> {
    /// Provides a reference to the underlying fixed-size byte array.
    #[inline(always)]
    fn as_ref(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize, const S: usize> AsRef<[u8]> for HexaUrlCore<N, S> {
    /// Provides a reference to the underlying bytes as a slice.
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize, const S: usize> Binary for HexaUrlCore<N, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<const N: usize, const S: usize> serde::Serialize for HexaUrlCore<N, S> {
        fn serialize<Ser: serde::Serializer>(
            &self,
            serializer: Ser,
        ) -> Result<Ser::Ok, Ser::Error> {
            if serializer.is_human_readable() {
                self.to_string().serialize(serializer)
            } else {
                serializer.serialize_bytes(self.as_bytes())
            }
        }
    }

    pub(crate) mod deserialize {
        use super::HexaUrlCore;
        use std::convert::TryFrom;

        #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
        pub(crate) struct HexaUrlVisitor<const N: usize, const S: usize>;

        #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
        #[allow(clippy::needless_lifetimes)]
        impl<'de, const N: usize, const S: usize> serde::de::Visitor<'de> for HexaUrlVisitor<N, S> {
            type Value = HexaUrlCore<N, S>;

            #[cfg_attr(coverage_nightly, coverage(off))]
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("bytes or string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                HexaUrlCore::new_quick(value).map_err(E::custom)
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                HexaUrlCore::try_from(value).map_err(E::custom)
            }
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de, const N: usize, const S: usize> serde::Deserialize<'de> for HexaUrlCore<N, S> {
        fn deserialize<D: serde::Deserializer<'de>>(
            deserializer: D,
        ) -> Result<HexaUrlCore<N, S>, D::Error> {
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
#[cfg_attr(docsrs, doc(cfg(feature = "arbitrary")))]
impl<'a, const N: usize, const S: usize> arbitrary::Arbitrary<'a> for HexaUrlCore<N, S> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        use crate::{decode::decode_core, validate::validate_with_config};

        let len = u.int_in_range(0..=N)?;
        let mut bytes = [0; N];
        u.fill_buffer(&mut bytes[..len])?;

        let mut dst = [0; S];
        let str = unsafe { str::from_utf8_unchecked(decode_core(&bytes, &mut dst)) };
        validate_with_config::<N>(str, Config::minimal())
            .map_err(|_| arbitrary::Error::IncorrectFormat)?;

        Ok(Self(bytes))
    }
}

#[cfg(feature = "candid")]
mod candid {
    use super::HexaUrlCore;
    use candid::{
        types::{Serializer, Type, TypeInner},
        CandidType,
    };

    #[cfg_attr(docsrs, doc(cfg(feature = "candid")))]
    impl<const N: usize, const S: usize> CandidType for HexaUrlCore<N, S> {
        fn _ty() -> Type {
            TypeInner::Vec(TypeInner::Nat8.into()).into()
        }
        fn idl_serialize<Ser>(&self, serializer: Ser) -> Result<(), Ser::Error>
        where
            Ser: Serializer,
        {
            serializer.serialize_blob(self.as_bytes())
        }
    }
}

#[cfg(feature = "ic-stable")]
mod ic {
    use super::HexaUrlCore;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    /// Implements the [`Storable`] trait for [`HexaUrlCore`] for use with Internet Computer stable structures.
    #[cfg_attr(docsrs, doc(cfg(feature = "ic-stable")))]
    impl<const N: usize, const S: usize> Storable for HexaUrlCore<N, S> {
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
    use serde_json;
    use std::collections::{BTreeMap, HashMap};

    /// Tests encoding and decoding of a string using the default configuration.
    #[test]
    fn test_encode_decode() {
        let input = "hello";
        let hexaurl = HexaUrlCore::<16, 21>::new(input).unwrap();
        let decoded = hexaurl.decode().unwrap();
        assert_eq!(input, decoded);
    }

    /// Tests encoding and decoding with minimal config
    #[test]
    fn test_encode_decode_minimal() {
        let input = "hello";
        let hexaurl = HexaUrlCore::<16, 21>::new_minimal_config(input).unwrap();
        let decoded = hexaurl.decode().unwrap();
        assert_eq!(input, decoded);
    }

    /// Tests the unchecked encoding and decoding of a string.
    #[test]
    fn test_encode_decode_unchecked() {
        unsafe {
            let input = "hello";
            let hexaurl = HexaUrlCore::<16, 21>::new_unchecked(input);
            let decoded = hexaurl.decode_unchecked();
            assert_eq!(input, decoded);
        }
    }

    /// Tests that `HexaUrl` implements the Hash trait properly by using it as a key in a HashMap.
    #[test]
    fn test_hash() {
        let input = "hello";
        let hexaurl = HexaUrlCore::<16, 21>::new(input).unwrap();
        let mut map = HashMap::new();
        map.insert(hexaurl, input);
        assert_eq!(map.get(&hexaurl), Some(&input));
    }

    /// Tests that `HexaUrl` implements ordering correctly by using it as a key in a BTreeMap.
    #[test]
    fn test_btree_map() {
        let input = "hello";
        let hexaurl = HexaUrlCore::<16, 21>::new(input).unwrap();
        let mut map = BTreeMap::new();
        map.insert(hexaurl, input);
        assert_eq!(map.get(&hexaurl), Some(&input));
    }

    /// Tests the ordering between two `HexaUrl` values created from different strings.
    #[test]
    fn test_ordering() {
        let input1 = "hello";
        let input2 = "world";
        let hexaurl1 = HexaUrlCore::<16, 21>::new(input1).unwrap();
        let hexaurl2 = HexaUrlCore::<16, 21>::new(input2).unwrap();
        assert!(hexaurl1 < hexaurl2);
        assert_eq!(hexaurl1 < hexaurl2, input1 < input2);
    }

    /// Tests successful creation of HexaUrl from a byte slice.
    #[test]
    fn test_try_from_bytes_success() {
        let input = "hello";
        let hexaurl = HexaUrlCore::<16, 21>::new(input).unwrap();
        let bytes = hexaurl.as_bytes();
        let hexaurl_copy = HexaUrlCore::<16, 21>::try_from(&bytes[..]).unwrap();
        assert_eq!(hexaurl, hexaurl_copy);
    }

    /// Tests that trying to create a HexaUrl from a byte slice with invalid length returns an error.
    #[test]
    fn test_try_from_bytes_invalid_length() {
        let bytes = [0u8; 15]; // Incorrect length
        let result = HexaUrlCore::<16, 21>::try_from(&bytes[..]);
        assert!(result.is_err());
    }

    /// Tests encoding and decoding using a specific configuration.
    #[test]
    fn test_new_with_config() {
        let input = "hello";
        let config = Config::minimal();
        let hexaurl = HexaUrlCore::<16, 21>::new_with_config(input, config).unwrap();
        let decoded = hexaurl.decode_with_config(config).unwrap();
        assert_eq!(input, decoded);
    }

    /// Tests the len() method of HexaUrlCore
    #[test]
    fn test_len() {
        let empty = HexaUrlCore::<16, 21>::new_minimal_config("").unwrap();
        assert_eq!(empty.len(), 0);

        let input = "hello";
        let hexaurl = HexaUrlCore::<16, 21>::new(input).unwrap();
        assert_eq!(hexaurl.len(), input.len());

        let long_input = "hello-world";
        let long_hexaurl = HexaUrlCore::<16, 21>::new(long_input).unwrap();
        assert_eq!(long_hexaurl.len(), long_input.len());
    }

/// Tests resizing to a larger capacity
    #[test]
    fn test_resize_larger() {
        let input = "hello";
        let small = HexaUrlCore::<8, 10>::new(input).unwrap();
        let large = small.resize::<16, 21>();
        assert_eq!(large.decode().unwrap(), input);
    }

    /// Tests resizing to a smaller capacity (with truncation)
    #[test]
    fn test_resize_smaller() {
        let input = "hello-world";
        let large = HexaUrlCore::<16, 21>::new(input).unwrap();
        let small = large.resize::<8, 10>();
        assert_eq!(small.decode().unwrap(), "hello-worl");
    }

    /// Tests reallocation to larger capacity
    #[test]
    fn test_reallocate_larger() {
        let input = "hello";
        let small = HexaUrlCore::<8, 10>::new(input).unwrap();
        let large = small.reallocate::<16, 21>().unwrap();
        assert_eq!(large.decode().unwrap(), input);
    }

    /// Tests reallocation to smaller capacity
    #[test]
    fn test_reallocate_smaller() {
        let input = "hello";
        let large = HexaUrlCore::<16, 21>::new(input).unwrap();
        let small = large.reallocate::<8, 10>().unwrap();
        assert_eq!(small.decode().unwrap(), input);
    }

    /// Tests reallocation failure when content is too large
    #[test]
    fn test_reallocate_too_large() {
        let input = "hello-world";
        let large = HexaUrlCore::<16, 21>::new(input).unwrap();
        assert!(large.reallocate::<8, 10>().is_none());
    }

    /// Tests try_from for String and &String
    #[test]
    fn test_try_from_string() {
        let input = String::from("hello");
        let hexaurl1 = HexaUrlCore::<16, 21>::try_from(input.clone()).unwrap();
        let hexaurl2 = HexaUrlCore::<16, 21>::try_from(&input).unwrap();
        assert_eq!(hexaurl1, hexaurl2);
    }

    /// Tests as_ref implementations
    #[test]
    fn test_as_ref() {
        let input = "hello";
        let hexaurl = HexaUrlCore::<16, 21>::new(input).unwrap();
        let _: &[u8; 16] = hexaurl.as_ref();
        let _: &[u8] = hexaurl.as_ref();
    }

    /// Tests Display implementation
    #[test]
    fn test_display() {
        let input = "hello";
        let hexaurl = HexaUrlCore::<16, 21>::new(input).unwrap();
        assert_eq!(hexaurl.to_string(), input);
    }

    /// Tests TryFrom<[u8; N]> implementation
    #[test]
    fn test_try_from_array() {
        let input = "hello";
        let hexaurl = HexaUrlCore::<16, 21>::new(input).unwrap();
        let bytes = *hexaurl.as_bytes();
        let hexaurl2 = HexaUrlCore::<16, 21>::try_from(bytes).unwrap();
        assert_eq!(hexaurl, hexaurl2);
    }

    /// Tests TryFrom<&[u8; N]> implementation
    #[test]
    fn test_try_from_array_ref() {
        let input = "hello";
        let hexaurl = HexaUrlCore::<16, 21>::new(input).unwrap();
        let bytes = hexaurl.as_bytes();
        let hexaurl2 = HexaUrlCore::<16, 21>::try_from(bytes).unwrap();
        assert_eq!(hexaurl, hexaurl2);
    }

    #[cfg(feature = "serde")]
    mod serde_impl {
        use super::*;

        /// Tests serialization and deserialization in a human-readable format.
        #[test]
        fn test_serde_serialization_human_readable() {
            let input = "hello";
            let hexaurl = HexaUrlCore::<16, 21>::new(input).unwrap();
            let json = serde_json::to_string(&hexaurl).unwrap();
            let deserialized: HexaUrlCore<16, 21> = serde_json::from_str(&json).unwrap();
            assert_eq!(hexaurl, deserialized);
        }

        /// Tests serialization in a non-human-readable context.
        #[test]
        fn test_serde_serialization_non_human_readable() {
            // Note: serde_json is always human-readable, so we simulate a non-human-readable serializer using bincode if available.
            // Here we only check that the process does not panic and round-trips correctly.
            let input = "hello";
            let hexaurl = HexaUrlCore::<16, 21>::new(input).unwrap();
            let encoded = bincode::serialize(&hexaurl).unwrap();
            let decoded: HexaUrlCore<16, 21> = bincode::deserialize(&encoded).unwrap();
            assert_eq!(hexaurl, decoded);
        }
    }

    #[cfg(feature = "candid")]
    mod candid_impl {
        use super::HexaUrlCore;
        use candid::{types::TypeInner, CandidType, Decode, Encode};

        /// Tests CandidType implementation
        #[test]
        fn test_candid_type() {
            assert_eq!(
                HexaUrlCore::<16, 21>::_ty(),
                TypeInner::Vec(TypeInner::Nat8.into()).into()
            );
        }

        /// Tests IDL serialization
        #[test]
        fn test_idl_serialization() {
            let input = "hello";
            let hexaurl = HexaUrlCore::<16, 21>::new(input).unwrap();
            let encoded = Encode!(&hexaurl);
            assert!(encoded.is_ok());
            let decoded = Decode!(&encoded.unwrap(), HexaUrlCore<16, 21>);
            assert!(decoded.is_ok());
            assert_eq!(hexaurl, decoded.unwrap());
        }
    }

    #[cfg(feature = "ic-stable")]
    mod storable_impl {
        use super::*;
        use ic_stable_structures::Storable;

        /// Tests Storable implementation
        #[test]
        fn test_storable() {
            let input = "hello";
            let hexaurl = HexaUrlCore::<16, 21>::new(input).unwrap();
            let bytes = hexaurl.to_bytes();
            let restored = HexaUrlCore::<16, 21>::from_bytes(bytes);
            assert_eq!(hexaurl, restored);
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
                let hexaurl = HexaUrlCore::<16, 21>::arbitrary(u)?;
                let decoded = hexaurl.decode_with_config(Config::minimal()).unwrap();
                assert_eq!(hexaurl.to_string(), decoded);
                Ok(())
            }
            arbtest(prop).budget_ms(1_000).run();
        }
    }
}
