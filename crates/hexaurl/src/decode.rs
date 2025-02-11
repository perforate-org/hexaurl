//! Functions for decoding HexaURL-encoded bytes back into strings.
//!
//! This module provides both checked and unchecked decoding functions. The safe functions perform validation
//! to ensure all HexaURL values are within the valid range, while the unchecked functions assume the input
//! is already valid for increased performance.

use crate::{ASCII_OFFSET, MASK_FOUR_BITS, MASK_SIX_BITS, MASK_TWO_BITS, SHIFT_FOUR_BITS, SHIFT_SIX_BITS, SHIFT_TWO_BITS};
use crate::Error;
use hexaurl_validate::{validate_with_config, config::Config};
use std::slice;

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
    let cfg = Config::default();
    decode_with_config::<N, S>(bytes, cfg)
}

#[inline]
pub fn decode_with_config<const N: usize, const S: usize>(bytes: &[u8; N], config: Config) -> Result<String, Error> {
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
/// # Examples
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
    // SAFETY: The function assumes the input is valid and does not contain any null bytes or spaces.
    unsafe { std::str::from_utf8_unchecked(slice) }.to_owned()
}

/// Converts a HexaURL alphabet character to its lowercase ASCII representation using bitwise operations.
///
/// If `byte` is between 33 and 58 (representing the uppercase HexaURL letters 'A' to 'Z'),
/// it converts it to its lowercase counterpart by adding 64. Otherwise, it adds the ASCII offset (32).
///
/// # Note
/// This function assumes the input is a valid HexaURL character.
#[inline(always)]
const fn convert_hexaurl_char_to_lowercase(byte: u8) -> u8 {
    #[allow(clippy::manual_range_contains)]
    if byte >= 33 && byte <= 58 {
        byte + 64
    } else {
        byte + ASCII_OFFSET
    }
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
pub(crate) fn decode_core<'a, const N: usize, const S: usize>(src: &[u8; N], dst: &'a mut [u8; S]) -> &'a [u8] {
    // The output size is 4/3 times the input size.
    assert_eq!(N * 4 / 3, S, "Output size mismatch");

    // SAFETY: Pointer operations on fixed-length byte strings are safe.
    unsafe {
        'outer: {
            // Process full 3-byte chunks first.
            for chunk_idx in 0..full_chunks(N) {
                // Compute pointers for the current chunk to avoid bounds checks.
                let src = slice::from_raw_parts(src.as_ptr().add(chunk_idx * 3), 3);
                let dst = slice::from_raw_parts_mut(dst.as_mut_ptr().add(chunk_idx * 4), 4);
                // If the chunk is empty, break the loop.
                if decode_chunk_sixbit(src, dst).is_err() {
                    break 'outer;
                }
            }
            // Process any remaining bytes that don't make up a complete 3-byte chunk.
            decode_remaining_sixbit(
                slice::from_raw_parts(src.as_ptr().add(remaining_ptr_offset(N)), remaining_bytes(N)),
                slice::from_raw_parts_mut(dst.as_mut_ptr().add(remaining_chars_ptr_offset(N)), remaining_bytes(N)), // len is same as remaining_bytes(N)
                remaining_bytes(N),
            );
        }
    }

    // Trim any trailing null bytes or spaces from the result.
    let trimmed_end = dst
        .iter()
        .rposition(|&c| c != b' ' && c != b'\0')
        .unwrap_or(S - 1);

    dst[..=trimmed_end].as_ref()
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

        *r = convert_hexaurl_char_to_lowercase((*s) >> SHIFT_TWO_BITS);
        *r.add(1) = convert_hexaurl_char_to_lowercase(((*s & MASK_TWO_BITS) << SHIFT_FOUR_BITS) | (*s.add(1) >> SHIFT_FOUR_BITS));
        *r.add(2) = convert_hexaurl_char_to_lowercase(((*s.add(1) & MASK_FOUR_BITS) << SHIFT_TWO_BITS) | (*s.add(2) >> SHIFT_SIX_BITS));
        *r.add(3) = convert_hexaurl_char_to_lowercase(*s.add(2) & MASK_SIX_BITS);
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
                *r = convert_hexaurl_char_to_lowercase((*s) >> SHIFT_TWO_BITS);
            }
            2 => {
                // If there are 2 remaining bytes:
                // First character: decode by shifting the first byte right by 2 bits.
                *r = convert_hexaurl_char_to_lowercase((*s) >> SHIFT_TWO_BITS);
                // Second character: combine the lower 2 bits of the first byte with the upper 4 bits of the second byte.
                *r.add(1) = convert_hexaurl_char_to_lowercase(((*s & MASK_TWO_BITS) << SHIFT_FOUR_BITS) | (*s.add(1) >> SHIFT_FOUR_BITS));
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
        let decoded = decode_with_config::<16, 21>(&encoded, config).expect("Decoding with config failed");
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
