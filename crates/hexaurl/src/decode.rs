//! Decoding Utilities
//!
//! This module provides both checked and unchecked decoding functions. The safe functions perform validation
//! to ensure all HexaURL values are within the valid range, while the unchecked functions assume the input
//! is already valid for increased performance.

use crate::{Error, MASK_FOUR_BITS, MASK_SIX_BITS, MASK_TWO_BITS, utils::len};
use hexaurl_validate::{config::Config, validate_with_config};
use std::{slice, str};

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
    decode_with_config::<N, S>(bytes, Config::default())
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
    config: Config,
) -> Result<String, Error> {
    let res = decode_unchecked::<N, S>(bytes);
    validate_with_config::<N>(&res, config)?;
    Ok(res)
}

/// This function performs decoding without validating whether the HexaURL values are within the
/// valid range or whether the resulting bytes form a valid UTF-8 string. Use this function only
/// when you are certain the input is valid to avoid undefined behavior.
///
/// # Safety
/// - The `bytes` slice must contain valid HexaURL-encoded data.
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
    let slice = decode_core::<N, S>(bytes, &mut res);
    // SAFETY: The function assumes the input is valid and does not contain any null bytes.
    unsafe { str::from_utf8_unchecked(slice) }.to_owned()
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

/// Calculates the number of remaining bytes after processing full chunks.
#[inline(always)]
const fn remaining_bytes(n: usize) -> usize {
    n % 3
}

/// Calculates the offset of the remaining bytes in the input.
#[inline(always)]
const fn remaining_ptr_offset(n: usize) -> usize {
    full_chunks(n) * 3
}

/// Calculates the offset of the remaining characters in the output.
#[inline(always)]
const fn remaining_chars_ptr_offset(n: usize) -> usize {
    full_chunks(n) * 4
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

    'outer: {
        // Process full 3-byte chunks first.
        for chunk_idx in 0..full_chunks(N) {
            // Compute pointers for the current chunk to avoid bounds checks.
            let src_chunk = unsafe { slice::from_raw_parts(src.as_ptr().add(chunk_idx * 3), 3) };
            let dst_chunk =
                unsafe { slice::from_raw_parts_mut(dst.as_mut_ptr().add(chunk_idx * 4), 4) };
            // If the chunk is empty, break the decoding.
            if decode_chunk_sixbit(src_chunk, dst_chunk).is_err() {
                break 'outer;
            }
        }
        // Process any remaining bytes that don't make up a complete 3-byte chunk.
        decode_remaining_sixbit(
            unsafe {
                slice::from_raw_parts(
                    src.as_ptr().add(remaining_ptr_offset(N)),
                    remaining_bytes(N),
                )
            },
            unsafe {
                slice::from_raw_parts_mut(
                    dst.as_mut_ptr().add(remaining_chars_ptr_offset(N)),
                    remaining_bytes(N), // len is same as remaining_bytes(N)
                )
            },
            remaining_bytes(N),
        );
    }

    // Trim any trailing null bytes from the result.
    dst[..len(dst)].as_ref()
}

/// Decodes a chunk of 3 bytes into 4 SIXBIT characters.
///
/// This function uses unsafe pointer arithmetic to extract and decode four SIXBIT characters from a block of three bytes.
/// It assumes that `src` has at least 3 bytes and `dst` has space for at least 4 bytes.
///
/// # Safety
/// This function performs unchecked pointer arithmetic and should be used only when the caller ensures the slices meet
/// the required minimum lengths.
///
/// # Parameters
/// - `src`: A slice of 3 bytes containing SIXBIT-encoded data.
/// - `dst`: A mutable slice of 4 bytes where the decoded characters will be stored.
#[inline(always)]
const fn decode_chunk_sixbit(src: &[u8], dst: &mut [u8]) -> Result<(), ()> {
    // Use unsafe pointer arithmetic to eliminate bounds checks.
    // We assume that src has at least 3 bytes and dst has at least 4 bytes.
    unsafe {
        let s = src.as_ptr();
        // If the first byte is zero, the chunk is empty.
        if *s == 0 {
            return Err(());
        }
        let r = dst.as_mut_ptr();

        *r = convert((*s) >> 2);
        *r.add(1) = convert(((*s & MASK_TWO_BITS) << 4) | (*s.add(1) >> 4));
        *r.add(2) = convert(((*s.add(1) & MASK_FOUR_BITS) << 2) | (*s.add(2) >> 6));
        *r.add(3) = convert(*s.add(2) & MASK_SIX_BITS);
    }
    Ok(())
}

/// Decodes any remaining SIXBIT-encoded bytes that do not form a full 3-byte chunk.
///
/// This function uses unsafe pointer arithmetic to extract and decode the remaining bytes based on the number of leftover bytes.
/// It assumes that `src` has at most 2 bytes and `dst` has space for at most 2 bytes.
///
/// # Safety
/// This function performs unchecked pointer arithmetic and should be used only when the caller ensures the slices meet
/// the required maximum lengths.
///
/// # Parameters
/// - `src`: A slice of at most 2 bytes containing SIXBIT-encoded data.
/// - `dst`: A mutable slice of at most 2 bytes where the decoded characters will be stored.
#[inline(always)]
const fn decode_remaining_sixbit(src: &[u8], dst: &mut [u8], remaining_bytes: usize) {
    // Decode remaining SIXBIT bytes based on the number of leftover bytes.
    unsafe {
        let s = src.as_ptr();
        let r = dst.as_mut_ptr();

        match remaining_bytes {
            1 => {
                // If there is 1 remaining byte, decode it by shifting right by 2 bits and converting to lowercase.
                *r = convert((*s) >> 2);
            }
            2 => {
                // If there are 2 remaining bytes:
                // First character: decode by shifting the first byte right by 2 bits.
                *r = convert((*s) >> 2);
                // Second character: combine the lower 2 bits of the first byte with the upper 4 bits of the second byte.
                *r.add(1) = convert(((*s & MASK_TWO_BITS) << 4) | (*s.add(1) >> 4));
            }
            _ => {
                // No decoding is performed if there are no remaining bytes or an unexpected number.
            }
        }
    }
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
        let config = hexaurl_validate::config::Config::default();
        let encoded: [u8; 16] = encode(original).expect("Encoding failed");
        let decoded =
            decode_with_config::<16, 21>(&encoded, config).expect("Decoding with config failed");
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
