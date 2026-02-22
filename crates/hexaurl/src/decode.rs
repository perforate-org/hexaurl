//! Decoding Utilities
//!
//! This module provides both checked and unchecked decoding functions. The safe functions perform validation
//! to ensure all HexaURL values are within the valid range, while the unchecked functions assume the input
//! is already valid for increased performance.

use crate::{Error, MASK_FOUR_BITS, MASK_SIX_BITS, MASK_TWO_BITS};
use hexaurl_validate::{config::Config, validate_with_config};
use std::str;

/// This function converts a slice of HexaURL-encoded bytes into the original string based on the provided length.
///
/// # Parameters
/// - `bytes`: A slice of bytes containing SIXBIT-encoded data.
///
/// # Examples
///
/// ```rust
/// use hexaurl::{encode, decode};
///
/// let input = "hello";
/// let encoded_bytes: [u8; 16] = encode(input).unwrap();
/// let decoded_string = decode::<16, 21>(&encoded_bytes).unwrap();
/// assert_eq!(decoded_string, input);
/// ```
#[inline]
pub fn decode<const N: usize, const S: usize>(bytes: &[u8; N]) -> Result<String, Error> {
    let config = Config::<N>::default();
    decode_with_config::<N, S>(bytes, &config)
}

/// Decodes a slice of HexaURL-encoded bytes into a string using a custom validation configuration.
///
/// # Parameters
/// - `bytes`: A reference to an array of bytes containing HexaURL-encoded data.
/// - `config`: A custom configuration for validating the decoded string.
///
/// # Returns
/// A `Result` containing the decoded string if validation succeeds, or an `Error` otherwise.
///
/// # Errors
/// Returns an `Error` if the decoded string fails to validate according to the provided configuration.
#[inline]
pub fn decode_with_config<const N: usize, const S: usize>(
    bytes: &[u8; N],
    config: &Config<N>,
) -> Result<String, Error> {
    let mut dst = [0u8; S];
    let res = decode_into_with_config::<N, S>(bytes, &mut dst, config)?;
    Ok(res.to_owned())
}

/// Decodes into a caller-provided buffer using default validation configuration.
///
/// Returns a borrowed string slice into `dst`, avoiding allocation in the decode path.
#[inline]
pub fn decode_into<'a, const N: usize, const S: usize>(
    bytes: &[u8; N],
    dst: &'a mut [u8; S],
) -> Result<&'a str, Error> {
    let config = Config::<N>::default();
    decode_into_with_config::<N, S>(bytes, dst, &config)
}

/// Decodes into a caller-provided buffer using a custom validation configuration.
///
/// Returns a borrowed string slice into `dst`, avoiding allocation in the decode path.
#[inline]
pub fn decode_into_with_config<'a, const N: usize, const S: usize>(
    bytes: &[u8; N],
    dst: &'a mut [u8; S],
    config: &Config<N>,
) -> Result<&'a str, Error> {
    let res = decode_core::<N, S>(bytes, dst);
    // SAFETY: decode_core only emits ASCII bytes from the lookup table, which are always valid UTF-8.
    let res = unsafe { str::from_utf8_unchecked(res) };
    validate_with_config::<N>(res, config)?;
    Ok(res)
}

/// This function performs decoding without running HexaURL validation checks.
/// It is faster than [`decode`] and [`decode_with_config`], but accepts any byte pattern.
///
/// If `bytes` does not contain a valid HexaURL payload, the returned string may be semantically
/// incorrect for your application because delimiter/length/composition rules are not enforced.
///
/// # Parameters
/// - `bytes`: A slice of bytes containing HexaURL-encoded data.
///
/// # Returns
/// The decoded string.
///
/// # Example
///
/// ```rust
/// use hexaurl::{encode, decode_unchecked};
///
/// let input = "hello";
/// let encoded_bytes: [u8; 16] = encode(input).unwrap();
/// let decoded_string = decode_unchecked::<16, 21>(&encoded_bytes);
/// assert_eq!(decoded_string, input);
/// ```
#[inline(always)]
pub fn decode_unchecked<const N: usize, const S: usize>(bytes: &[u8; N]) -> String {
    let mut res: [u8; S] = [0; S];
    decode_unchecked_into::<N, S>(bytes, &mut res).to_owned()
}

/// Decodes into a caller-provided buffer without validation checks.
///
/// Returns a borrowed string slice into `dst`, avoiding allocation in the decode path.
#[inline(always)]
pub fn decode_unchecked_into<'a, const N: usize, const S: usize>(
    bytes: &[u8; N],
    dst: &'a mut [u8; S],
) -> &'a str {
    let slice = decode_core::<N, S>(bytes, dst);
    // SAFETY: decode_core only emits ASCII bytes from the lookup table, which are always valid UTF-8.
    unsafe { str::from_utf8_unchecked(slice) }
}

// ============================================================
//
//            HexaURL Core Decoding Logic
//
// This section implements the central routines used to decode HexaURL-encoded bytes
// into their corresponding ASCII representations. It processes the input in full 3-byte
// chunks where possible and handles any remaining bytes appropriately, using unsafe
// pointer arithmetic for performance while relying on prior validations for safety.
//
// ============================================================

/// Index-based lookup table mapping encoded 6-bit values to their corresponding ASCII byte values.
/// Invalid indices are set to 0 (null character).
#[rustfmt::skip]
const LOOKUP_TABLE: [u8; 64] = [
      0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,  45,   0,   0,
     48,  49,  50,  51,  52,  53,  54,  55,  56,  57,   0,   0,   0,   0,   0,   0,
      0,  97,  98,  99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,   0,   0,   0,   0,  95,
];

/// Converts a HexaURL alphabet character to its lowercase ASCII representation using bitwise operations.
///
/// If `byte` is between 33 and 58 (representing the uppercase HexaURL letters 'A' to 'Z'),
/// it converts it to its lowercase counterpart by adding 64. Otherwise, it adds the ASCII offset (32).
///
/// # Note
/// This function assumes the input is a valid HexaURL character.
#[inline]
const unsafe fn convert(byte: u8) -> u8 {
    unsafe { LOOKUP_TABLE.as_ptr().add(byte as usize).read() }
}

/// Calculates the number of full 3-byte chunks in the input.
#[inline(always)]
const fn full_chunks(n: usize) -> usize {
    n / 3
}

/// Decodes a fixed-size array of HexaURL-encoded bytes into a String.
///
/// This function uses a fixed-size stack allocated array to avoid heap allocation overhead.
/// It processes the input bytes in full 3-byte chunks first and then decodes any remaining bytes
/// that do not form a complete chunk. The decoding leverages unsafe pointer arithmetic for performance.
/// Any trailing null bytes or spaces in the resulting byte array are trimmed before converting to a UTF-8 string.
#[inline(always)]
pub(crate) fn decode_core<'a, const N: usize, const S: usize>(
    src: &[u8; N],
    dst: &'a mut [u8; S],
) -> &'a [u8] {
    // The output size is 4/3 times the input size.
    assert_eq!(N * 4 / 3, S, "Output size mismatch");

    let src_ptr = src.as_ptr();
    let dst_ptr = dst.as_mut_ptr();
    let chunks = full_chunks(N);
    let mut decoded_len = 0usize;

    // Process full 3-byte chunks first.
    for chunk_idx in 0..chunks {
        unsafe {
            let s = src_ptr.add(chunk_idx * 3);
            if *s == 0 {
                return dst[..decoded_len].as_ref();
            }

            let r = dst_ptr.add(chunk_idx * 4);
            let v0 = convert((*s) >> 2);
            let v1 = convert(((*s & MASK_TWO_BITS) << 4) | (*s.add(1) >> 4));
            let v2 = convert(((*s.add(1) & MASK_FOUR_BITS) << 2) | (*s.add(2) >> 6));
            let v3 = convert(*s.add(2) & MASK_SIX_BITS);

            *r = v0;
            *r.add(1) = v1;
            *r.add(2) = v2;
            *r.add(3) = v3;

            let base = chunk_idx * 4;
            if v0 != 0 {
                decoded_len = base + 1;
            }
            if v1 != 0 {
                decoded_len = base + 2;
            }
            if v2 != 0 {
                decoded_len = base + 3;
            }
            if v3 != 0 {
                decoded_len = base + 4;
            }
        }
    }

    // Process remaining bytes without creating temporary slices.
    let rem = N % 3;
    let rem_base = chunks * 4;
    if rem > 0 {
        unsafe {
            let s = src_ptr.add(chunks * 3);
            let r = dst_ptr.add(rem_base);
            let v0 = convert((*s) >> 2);
            *r = v0;
            if v0 != 0 {
                decoded_len = rem_base + 1;
            }

            if rem == 2 {
                let v1 = convert(((*s & MASK_TWO_BITS) << 4) | (*s.add(1) >> 4));
                *r.add(1) = v1;
                if v1 != 0 {
                    decoded_len = rem_base + 2;
                }
            }
        }
    }
    dst[..decoded_len].as_ref()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode::encode;

    #[test]
    fn test_encode_and_decode() {
        let original = "hello-hexaurl";
        let encoded: [u8; 16] = encode(original).expect("Encoding failed");
        let decoded = decode::<16, 21>(&encoded).expect("Decoding failed");
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_encode_and_decode_with_config() {
        let original = "Test-Config";
        let config = hexaurl_validate::config::Config::<16>::default();
        let encoded: [u8; 16] = encode(original).expect("Encoding failed");
        let decoded =
            decode_with_config::<16, 21>(&encoded, &config).expect("Decoding with config failed");
        assert_eq!(original.to_ascii_lowercase(), decoded);
    }

    #[test]
    fn test_decode_unchecked() {
        let original = "Unchecked-Test";
        let encoded: [u8; 16] = encode(original).expect("Encoding failed");
        let decoded = decode_unchecked::<16, 21>(&encoded);
        assert_eq!(original.to_ascii_lowercase(), decoded);
    }
}
