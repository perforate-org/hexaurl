//! Encoding Utilities
//!
//! This module provides functions to encode strings to the HexaURL format, which packs characters into a compact
//! SIXBIT representation. There are both safe and unsafe variants:
//!
//! • The safe functions ([`encode`], [`encode_with_config`]) perform runtime
//!   validation to ensure each character falls within the legal SIXBIT range.
//!
//! • The unsafe functions ([`encode_quick_checked`], [`encode_unchecked`]) omit validation for speed, and it is the caller's responsibility to
//!   guarantee that the input contains only valid HexaURL characters (ASCII 45, 48-57, 65-90, 95).
//!
//! All functions return a fixed-size byte array containing the packed result.

use crate::{Error, MASK_FOUR_BITS, MASK_TWO_BITS};
use hexaurl_config::Config;
use hexaurl_validate::{check_encoding_safe, validate, validate_with_config};

/// Encodes the input string into a compact HexaURL representation using default validation rules.
///
/// This function validates that all characters in the string are within the allowed SIXBIT range and then encodes the string.
/// It returns a fixed-size byte array containing the encoded result.
///
/// # Arguments
///
/// * `input` - A string slice that holds the data to be encoded.
///
/// # Returns
///
/// * `Ok([u8; N])` containing the encoded byte array if validation passes.
/// * `Err(Error)` if the input contains invalid characters.
///
/// # Examples
///
/// ```rust
/// use hexaurl::encode;
///
/// let input = "hello";
/// let encoded_bytes: [u8; 16] = encode(input).unwrap();
/// ```
#[inline]
pub fn encode<const N: usize>(input: &str) -> Result<[u8; N], Error> {
    unsafe {
        validate::<N>(input)?;
        Ok(encode_core(input))
    }
}

/// Encodes the input string into a HexaURL representation using a custom validation configuration.
///
/// This function validates and encodes the input string similar to [`encode`], but allows specifying a custom
/// [`Config`] for more control over the validation process.
///
/// # Arguments
///
/// * `input` - A string slice holding the data to be encoded.
/// * `config` - A [`Config`] instance that customizes the validation criteria.
///
/// # Returns
///
/// * `Ok([u8; N])` containing the encoded data if the input is valid.
/// * `Err(Error)` if validation fails.
#[inline]
pub fn encode_with_config<const N: usize>(input: &str, config: Config) -> Result<[u8; N], Error> {
    unsafe {
        validate_with_config::<N>(input, config)?;
        Ok(encode_core(input))
    }
}

/// Performs a simple validation check before encoding the input string into HexaURL format.
///
/// The function performs a fast check (without detailed error messages) to ensure that the input string is safe for encoding and avoids collisions.
/// If the input passes the safety check, it encodes the string; otherwise, it returns `None`.
///
/// # Arguments
///
/// * `input` - A string slice to be encoded.
///
/// # Returns
///
/// * `Some([u8; N])` if the input is safe and encoding is performed.
/// * `None` if the input fails the quick check.
#[inline(always)]
pub fn encode_quick_checked<const N: usize>(input: &str) -> Result<[u8; N], Error> {
    check_encoding_safe::<N>(input)?;
    unsafe { Ok(encode_core(input)) }
}

/// Encodes the input string into HexaURL format without performing any validation checks.
///
/// # Safety
///
/// The input string must be valid ASCII.
///
/// # Arguments
///
/// * `input` - A string slice that is assumed to be valid for HexaURL encoding.
///
/// # Returns
///
/// * A fixed-size byte array ([u8; N]) containing the encoded result.
///
/// # Examples
///
/// ```rust
/// use hexaurl::encode_unchecked;
///
/// unsafe {
///     let input = "hello";
///     let encoded_bytes: [u8; 16] = encode_unchecked(input);
/// }
/// ```
#[inline(always)]
pub unsafe fn encode_unchecked<const N: usize>(input: &str) -> [u8; N] {
    unsafe { encode_core(input) }
}

const LOOKUP_TABLE: [u8; 128] = [
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 13,  0,  0,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25,  0,  0,  0,  0,  0,  0,
     0, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58,  0,  0,  0,  0, 63,
     0, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58,  0,  0,  0,  0,  0,
];

/// Subtracts either 64 (for lowercase) or 32 (for others) from an ASCII character to prepare for SIXBIT encoding.
///
/// This function checks if `byte` is in the range 'a' to 'z'. If so, subtracts 64 to convert and normalize in one step,
/// otherwise subtracts 32 for other characters.
///
/// # Note
/// This function assumes the input is an ASCII character.
#[inline]
const unsafe fn convert(byte: u8) -> u8 {
    unsafe { LOOKUP_TABLE.as_ptr().add(byte as usize).read() }
}

/// Core function that performs the HexaURL encoding of an input string.
///
/// This function splits the input into 4-character chunks, converts each character into its SIXBIT representation,
/// and packs them into a byte array following the HexaURL encoding scheme. For the last chunk with fewer than 4 characters,
/// the output is padded appropriately.
///
/// # Safety
///
/// The input string must be valid ASCII.
///
/// # Arguments
///
/// * `input` - The string slice to encode.
///
/// # Returns
///
/// * A fixed-size byte array ([u8; N]) containing the packed HexaURL representation.
#[inline(always)]
unsafe fn encode_core<const N: usize>(input: &str) -> [u8; N] {
    let len = input.len();
    let mut bytes = [0u8; N];

    let full_chunks = len / 4;
    let remaining = len % 4;

    for chunk_idx in 0..full_chunks {
        let start = chunk_idx * 4;
        let chunk = &input.as_bytes()[start..start + 4];

        unsafe {
            // Convert each character to its SIXBIT value by converting to uppercase and subtracting ASCII_OFFSET.
            let a = convert(chunk[0]);
            let b = convert(chunk[1]);
            let c = convert(chunk[2]);
            let d = convert(chunk[3]);

            let byte_idx = chunk_idx * 3;

            // Pack 4 SIXBIT values into 3 bytes.
            bytes[byte_idx] = (a << 2) | (b >> 4);
            bytes[byte_idx + 1] = ((b & MASK_FOUR_BITS) << 4) | (c >> 2);
            bytes[byte_idx + 2] = ((c & MASK_TWO_BITS) << 6) | d;
        }
    }

    // Process any remaining characters that don't make up a complete 4-character chunk.
    if remaining > 0 {
        let start = full_chunks * 4;
        let chunk = &input.as_bytes()[start..];
        let byte_idx = full_chunks * 3;

        unsafe {
            match chunk.len() {
                3 => {
                    let a = convert(chunk[0]);
                    let b = convert(chunk[1]);
                    let c = convert(chunk[2]);

                    // Pack 3 SIXBIT values into 3 bytes (the last byte is padded).
                    bytes[byte_idx] = (a << 2) | (b >> 4);
                    bytes[byte_idx + 1] = ((b & MASK_FOUR_BITS) << 4) | (c >> 2);
                    bytes[byte_idx + 2] = (c & MASK_TWO_BITS) << 6;
                }
                2 => {
                    let a = convert(chunk[0]);
                    let b = convert(chunk[1]);

                    // Pack 2 SIXBIT values into 2 bytes (with padding in the second byte).
                    bytes[byte_idx] = (a << 2) | (b >> 4);
                    bytes[byte_idx + 1] = (b & MASK_FOUR_BITS) << 4;
                }
                1 => {
                    let a = convert(chunk[0]);

                    // Pack a single SIXBIT value into 1 byte (with padding).
                    bytes[byte_idx] = a << 2;
                }
                _ => unreachable!(),
            }
        }
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    use hexaurl_config::Config;

    #[test]
    fn test_encode_valid_input() {
        let input = "hello";
        let encoded = encode::<16>(input).unwrap();
        assert_eq!(encoded.len(), 16);
    }

    #[test]
    fn test_encode_with_config() {
        let input = "world";
        let config = Config::default();
        let encoded = encode_with_config::<16>(input, config).unwrap();
        assert_eq!(encoded.len(), 16);
    }

    #[test]
    fn test_encode_quick_checked_valid() {
        let input = "test";
        let encoded_opt = encode_quick_checked::<16>(input);
        assert!(encoded_opt.is_ok());
        let encoded = encoded_opt.unwrap();
        assert_eq!(encoded.len(), 16);
    }

    #[test]
    fn test_encode_quick_checked_invalid() {
        // Using '😃' which is not in the allowed SIXBIT range.
        let input = "invalid😃";
        let encoded_opt = encode_quick_checked::<16>(input);
        assert!(encoded_opt.is_err());
    }

    #[test]
    fn test_encode_unchecked() {
        unsafe {
            // This test assumes the caller guarantees valid characters.
            let input = "abcABC";
            let encoded = encode_unchecked::<16>(input);
            assert_eq!(encoded.len(), 16);
        }
    }

    #[test]
    fn test_encode_valid_non16() {
        let input = "test";
        let encoded = encode::<12>(input).unwrap();
        assert_eq!(encoded.len(), 12);

        let input2 = "hello-world";
        let encoded2 = encode::<20>(input2).unwrap();
        assert_eq!(encoded2.len(), 20);
    }

    #[test]
    fn test_encode_with_config_non16() {
        let input = "world";
        let config = Config::default();
        let encoded = encode_with_config::<12>(input, config).unwrap();
        assert_eq!(encoded.len(), 12);
    }

    #[test]
    fn test_encode_quick_checked_non16() {
        let input = "abc";
        let encoded_opt = encode_quick_checked::<9>(input);
        assert!(encoded_opt.is_ok());
        let encoded = encoded_opt.unwrap();
        assert_eq!(encoded.len(), 9);
    }
}
