//! Functions for encoding strings into HexaURL format.
//!
//! This module provides both safe and unsafe encoding functions. The safe functions perform validation
//! to ensure all characters are within the valid SIXBIT range, while the unsafe functions assume the input
//! is already valid for increased performance.

use crate::{Error, MASK_FOUR_BITS, MASK_TWO_BITS, ASCII_OFFSET, SHIFT_TWO_BITS, SHIFT_FOUR_BITS, SHIFT_SIX_BITS};
use hexaurl_config::validate::ValidationConfig;
use hexaurl_validate::validate;

fn to_uppercase(byte: u8) -> u8 {
    #[allow(clippy::manual_range_contains)]
    if byte >= b'a' && byte <= b'z' {
        byte - 32
    } else {
        byte
    }
}

/// This function converts the input string into a compact HexaURL-encoded byte vector and returns
/// the encoded bytes along with the original string length.
///
/// # Constraints
/// - Only accepts uppercase alphanumeric, hyphen, or underscore.
///
/// # Errors
/// Returns an [`Error::InvalidCharacter`] if the input contains characters invalid for HexaURL encoding.
///
/// # Examples
///
/// ```rust
/// use hexaurl::encode;
///
/// let input = "HELLO";
/// let encoded_bytes = encode(input, None).unwrap();
/// ```
#[inline]
pub fn encode(input: &str, config: Option<ValidationConfig>) -> Result<[u8; 16], Error> {
    validate(input, config)?;

    Ok(encode_core(input))
}

/// This function performs encoding without validating whether the input string contains only
/// valid HexaURL characters (ASCII 45, 48-57, 65-90, 95). Use this function only when you are certain the input
/// meets the required constraints to avoid undefined behavior.
///
/// # Safety
/// The caller must ensure that all characters in `str` are within the valid HexaURL range.
///
/// # Examples
///
/// ```rust
/// use hexaurl::encode_unchecked;
///
/// let input = "HELLO";
/// let encoded_bytes = encode_unchecked(input);
/// ```
#[inline(always)]
pub fn encode_unchecked(input: &str) -> [u8; 16] {
    encode_core(input)
}

#[inline(always)]
fn encode_core(input: &str) -> [u8; 16] {
    let len = input.len();
    let mut bytes = [0u8; 16];

    let full_chunks = len / 4;
    let remaining = len % 4;

    for chunk_idx in 0..full_chunks {
        let start = chunk_idx * 4;
        let chunk = &input.as_bytes()[start..start + 4];

        // Convert to SIXBIT values by subtracting ASCII_OFFSET directly
        let a = to_uppercase(chunk[0]) - ASCII_OFFSET;
        let b = to_uppercase(chunk[1]) - ASCII_OFFSET;
        let c = to_uppercase(chunk[2]) - ASCII_OFFSET;
        let d = to_uppercase(chunk[3]) - ASCII_OFFSET;

        let byte_idx = chunk_idx * 3;

        // Pack 4 SIXBIT values into 3 bytes
        bytes[byte_idx] = (a << SHIFT_TWO_BITS) | (b >> SHIFT_FOUR_BITS);
        bytes[byte_idx + 1] = ((b & MASK_FOUR_BITS) << SHIFT_FOUR_BITS) | (c >> SHIFT_TWO_BITS);
        bytes[byte_idx + 2] = ((c & MASK_TWO_BITS) << SHIFT_SIX_BITS) | d;
    }

    // Handle the remaining 1-3 characters, if any
    if remaining > 0 {
        let start = full_chunks * 4;
        let chunk = &input.as_bytes()[start..];
        let byte_idx = full_chunks * 3;

        match chunk.len() {
            3 => {
                // Convert to SIXBIT values by subtracting ASCII_OFFSET directly
                let a = to_uppercase(chunk[0]) - ASCII_OFFSET;
                let b = to_uppercase(chunk[1]) - ASCII_OFFSET;
                let c = to_uppercase(chunk[2]) - ASCII_OFFSET;

                // Pack 3 SIXBIT values into 2.25 bytes (rounded up to 3 bytes)
                bytes[byte_idx] = (a << SHIFT_TWO_BITS) | (b >> SHIFT_FOUR_BITS);
                bytes[byte_idx + 1] = ((b & MASK_FOUR_BITS) << SHIFT_FOUR_BITS) | (c >> SHIFT_TWO_BITS);
                bytes[byte_idx + 2] = (c & MASK_TWO_BITS) << SHIFT_SIX_BITS;
            },
            2 => {
                // Convert to SIXBIT values by subtracting ASCII_OFFSET directly
                let a = to_uppercase(chunk[0]) - ASCII_OFFSET;
                let b = to_uppercase(chunk[1]) - ASCII_OFFSET;

                // Pack 2 SIXBIT values into 1.5 bytes (rounded up to 2 bytes)
                bytes[byte_idx] = (a << SHIFT_TWO_BITS) | (b >> SHIFT_FOUR_BITS);
                bytes[byte_idx + 1] = (b & MASK_FOUR_BITS) << SHIFT_FOUR_BITS;
            },
            1 => {
                // Convert to SIXBIT value by subtracting ASCII_OFFSET directly
                let a = to_uppercase(chunk[0]) - ASCII_OFFSET;

                // Pack 1 SIXBIT value into 0.75 bytes (rounded up to 1 byte)
                bytes[byte_idx] = a << SHIFT_TWO_BITS;
            },
            _ => unreachable!(),
        }
    }

    bytes
}
