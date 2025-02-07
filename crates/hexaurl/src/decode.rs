//! Functions for decoding HexaURL-encoded bytes back into strings.
//!
//! This module provides both checked and unchecked decoding functions. The safe functions perform validation
//! to ensure all HexaURL values are within the valid range, while the unchecked functions assume the input
//! is already valid for increased performance.

use crate::{ASCII_OFFSET, MASK_FOUR_BITS, MASK_SIX_BITS, MASK_TWO_BITS, SHIFT_FOUR_BITS, SHIFT_SIX_BITS, SHIFT_TWO_BITS};
use crate::Error;
use hexaurl_validate::{validate, config::{Config, CaseType}};
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
/// let input = "HELLO";
/// let encoded_bytes = encode(input, None).unwrap();
/// let decoded_string = decode(&encoded_bytes, None).unwrap();
/// assert_eq!(decoded_string, input);
/// ```
#[inline]
pub fn decode(bytes: &[u8; 16], config: Option<Config>) -> Result<String, Error> {
    let mut res = decode_core(bytes);
    let cfg = config.unwrap_or_default();
    validate(&res, Some(cfg.validation()))?;
    if cfg.case() == CaseType::Lower {
        res.make_ascii_lowercase();
    }
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
/// let input = "HELLO";
/// let encoded_bytes = encode(input, None).unwrap();
/// let decoded_string = decode_unchecked(&encoded_bytes);
/// assert_eq!(decoded_string, input);
/// ```
#[inline(always)]
pub fn decode_unchecked(bytes: &[u8; 16]) -> String {
    decode_core(bytes)
}

#[inline(always)]
fn decode_core(bytes: &[u8; 16]) -> String {
    // Use a fixed-size array on the stack to avoid heap allocation overhead.
    let mut result: [u8; 21] = [0; 21];

    // SAFETY: Pointer operations on fixed-length byte strings are safe.
    unsafe {
        'decode: {
            // Decode the first 15 bytes into 20 SIXBIT characters.
            for chunk_idx in 0..5 {
                // Compute pointers for the current chunk to avoid bounds checks.
                let src = slice::from_raw_parts(bytes.as_ptr().add(chunk_idx * 3), 3);
                let dst = slice::from_raw_parts_mut(result.as_mut_ptr().add(chunk_idx * 4), 4);
                // If the chunk is empty, break the loop.
                if decode_chunk_sixbit(src, dst).is_err() {
                    break 'decode;
                }
            }
            // Decode the last byte directly into the final HexaURL character.
            decode_first_sixbit(*bytes.get_unchecked(15), &mut *result.as_mut_ptr().add(20));
        }

        // Trim any trailing null bytes or spaces from the result.
        let trimmed_end = result
            .iter()
            .rposition(|&c| c != b' ' && c != b'\0')
            .unwrap_or(20);
        let s = str::from_utf8_unchecked(&result[..=trimmed_end]);
        s.to_owned()
    }
}

/// Decodes the first SIXBIT character from the given byte.
///
/// This function extracts the upper six bits of `byte` and adds the ASCII offset to produce the decoded character.
///
/// # Parameters
/// - `byte`: The source byte containing a SIXBIT-encoded value.
/// - `dst`: A mutable reference where the resulting decoded character will be stored.
#[inline(always)]
const fn decode_first_sixbit(byte: u8, dst: &mut u8) {
    *dst = (byte >> SHIFT_TWO_BITS).wrapping_add(ASCII_OFFSET);
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

        *r = ((*s) >> SHIFT_TWO_BITS).wrapping_add(ASCII_OFFSET);
        *r.add(1) = (((*s & MASK_TWO_BITS) << SHIFT_FOUR_BITS) | (*s.add(1) >> SHIFT_FOUR_BITS))
            .wrapping_add(ASCII_OFFSET);
        *r.add(2) = (((*s.add(1) & MASK_FOUR_BITS) << SHIFT_TWO_BITS) | (*s.add(2) >> SHIFT_SIX_BITS))
            .wrapping_add(ASCII_OFFSET);
        *r.add(3) = (*s.add(2) & MASK_SIX_BITS).wrapping_add(ASCII_OFFSET);
    }
    Ok(())
}
