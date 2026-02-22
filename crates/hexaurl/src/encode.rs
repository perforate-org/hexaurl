//! Encoding Utilities
//!
//! This module provides functions to encode strings to the HexaURL format, which packs characters into a compact
//!
//! All functions return a fixed-size byte array containing the packed result.

use crate::{Error, MASK_FOUR_BITS, MASK_TWO_BITS};
use hexaurl_config::{Composition, Config};
use hexaurl_validate::check_encoding_safe;

/// Calculates the maximum length of the input string based on the number of output bytes.
#[inline(always)]
const fn calc_str_len(n: usize) -> usize {
    n * 4 / 3
}

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
    let config = Config::<N>::default();
    encode_with_config::<N>(input, &config)
}

/// Encodes the input string into a HexaURL representation using a custom validation configuration.
///
/// This function validates and encodes the input string similar to [`encode`], but allows specifying a custom
/// [`Config`] for more control over the validation process.
///
/// # Arguments
///
/// - `input` - A string slice holding the data to be encoded.
/// - `config` - A [`Config`] instance that customizes the validation criteria.
///
/// # Returns
///
/// - `Ok([u8; N])` containing the encoded data if the input is valid.
/// - `Err(Error)` if validation fails.
#[inline]
pub fn encode_with_config<const N: usize>(
    input: &str,
    config: &Config<N>,
) -> Result<[u8; N], Error> {
    encode_core_validated_with_config::<N>(input, config)
}

/// Encodes the input string into a compact HexaURL representation using minimal validation rules.
pub fn encode_minimal_config<const N: usize>(input: &str) -> Result<[u8; N], Error> {
    encode_core_minimal_validated::<N>(input)
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
pub fn encode_quick<const N: usize>(input: &str) -> Result<[u8; N], Error> {
    check_encoding_safe::<N>(input)?;
    unsafe { Ok(encode_core(input)) }
}

/// Encodes the input string into HexaURL format without performing any validation checks.
///
/// # Safety
///
/// <div class="warning">The input string must be ASCII. Otherwise, it causes undefined behavior.</div>
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

// ============================================================
//
//            HexaURL Core Encoding Logic
//
// This section implements the core logic that converts an input ASCII string into its
// compact HexaURL byte representation. The encoding algorithm splits the input into
// 4-character blocks, converts each character into its SIXBIT value using a lookup table,
// and then packs these values into bytes with the appropriate bit shifts. For input strings
// whose length is not a multiple of four, the remaining characters are processed and padded
// accordingly to produce a consistent, fixed-size output.
//
// ============================================================

/// Index-based lookup table mapping ASCII characters to their corresponding values in the HexaURL encoding scheme.
/// Invalid indices are set to 0 (null character).
#[rustfmt::skip]
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

#[inline(always)]
fn sixbit_value(byte: u8) -> Option<u8> {
    if byte >= 128 {
        return None;
    }
    let val = LOOKUP_TABLE[byte as usize];
    if val == 0 {
        None
    } else {
        Some(val)
    }
}

#[inline(always)]
fn encode_core_minimal_validated<const N: usize>(input: &str) -> Result<[u8; N], Error> {
    if input.len() > calc_str_len(N) {
        return Err(Error::StringTooLong(calc_str_len(N)));
    }

    encode_core_validated_inner::<N>(
        input.as_bytes(),
        true,
        true,
        hexaurl_config::DelimiterRules::default(),
        None,
        false,
        false,
    )
}

#[inline(always)]
fn encode_core_validated_with_config<const N: usize>(
    input: &str,
    config: &Config<N>,
) -> Result<[u8; N], Error> {
    let len = input.len();

    if let Some(min) = config.min_length() {
        if len < min {
            return Err(Error::StringTooShort(min));
        }
    }

    if len > config.effective_max() {
        return Err(Error::StringTooLong(config.effective_max()));
    }

    let delimiter_rules = config.delimiter_rules();
    let allow_hyphen = config.allow_hyphen();
    let allow_underscore = config.allow_underscore();

    encode_core_validated_inner::<N>(
        input.as_bytes(),
        allow_hyphen,
        allow_underscore,
        delimiter_rules,
        Some(config.composition()),
        delimiter_rules.allow_consecutive_hyphens(),
        delimiter_rules.allow_consecutive_underscores(),
    )
}

#[inline(always)]
fn encode_core_validated_inner<const N: usize>(
    input: &[u8],
    allow_hyphen: bool,
    allow_underscore: bool,
    delimiter_rules: hexaurl_config::DelimiterRules,
    composition: Option<Composition>,
    allow_consecutive_hyphens: bool,
    allow_consecutive_underscores: bool,
) -> Result<[u8; N], Error> {
    let len = input.len();
    let mut bytes = [0u8; N];

    let mut first_byte: u8 = 0;
    let mut last_byte: u8 = 0;
    let mut pending_delim_error: Option<Error> = None;
    let mut last_delim: Option<u8> = None;

    let full_chunks = len / 4;
    let remaining = len % 4;

    for chunk_idx in 0..full_chunks {
        let start = chunk_idx * 4;
        let chunk = &input[start..start + 4];

        if start == 0 {
            first_byte = chunk[0];
        }
        last_byte = chunk[3];

        let mut vals = [0u8; 4];
        for (i, &b) in chunk.iter().enumerate() {
            if b == b'-' {
                if !allow_hyphen {
                    return Err(Error::InvalidCharacter);
                }
            } else if b == b'_' && !allow_underscore {
                return Err(Error::InvalidCharacter);
            }

            let Some(v) = sixbit_value(b) else {
                return Err(Error::InvalidCharacter);
            };
            vals[i] = v;

            if pending_delim_error.is_none() {
                if let Some(comp) = composition {
                    match comp {
                        Composition::Alphanumeric => {}
                        Composition::AlphanumericHyphen => {
                            if b == b'-' {
                                if last_delim == Some(b'-') && !allow_consecutive_hyphens {
                                    pending_delim_error = Some(Error::ConsecutiveHyphens);
                                }
                                last_delim = Some(b'-');
                            } else {
                                last_delim = None;
                            }
                        }
                        Composition::AlphanumericUnderscore => {
                            if b == b'_' {
                                if last_delim == Some(b'_') && !allow_consecutive_underscores {
                                    pending_delim_error = Some(Error::ConsecutiveUnderscores);
                                }
                                last_delim = Some(b'_');
                            } else {
                                last_delim = None;
                            }
                        }
                        Composition::AlphanumericHyphenUnderscore => match b {
                            b'-' | b'_' => {
                                if let Some(prev) = last_delim {
                                    if prev == b {
                                        if b == b'-' && !allow_consecutive_hyphens {
                                            pending_delim_error = Some(Error::ConsecutiveHyphens);
                                        }
                                        if b == b'_' && !allow_consecutive_underscores {
                                            pending_delim_error =
                                                Some(Error::ConsecutiveUnderscores);
                                        }
                                    } else if !delimiter_rules.allow_adjacent_hyphen_underscore() {
                                        pending_delim_error = Some(Error::AdjacentHyphenUnderscore);
                                    }
                                }
                                last_delim = Some(b);
                            }
                            _ => {
                                last_delim = None;
                            }
                        },
                    }
                }
            } else if let Some(comp) = composition {
                match comp {
                    Composition::AlphanumericHyphen => {
                        last_delim = if b == b'-' { Some(b'-') } else { None };
                    }
                    Composition::AlphanumericUnderscore => {
                        last_delim = if b == b'_' { Some(b'_') } else { None };
                    }
                    Composition::AlphanumericHyphenUnderscore => {
                        last_delim = match b {
                            b'-' | b'_' => Some(b),
                            _ => None,
                        };
                    }
                    Composition::Alphanumeric => {}
                }
            }
        }

        let byte_idx = chunk_idx * 3;
        let a = vals[0];
        let b = vals[1];
        let c = vals[2];
        let d = vals[3];
        bytes[byte_idx] = (a << 2) | (b >> 4);
        bytes[byte_idx + 1] = ((b & MASK_FOUR_BITS) << 4) | (c >> 2);
        bytes[byte_idx + 2] = ((c & MASK_TWO_BITS) << 6) | d;
    }

    if remaining > 0 {
        let start = full_chunks * 4;
        let chunk = &input[start..];

        if start == 0 {
            first_byte = chunk[0];
        }
        last_byte = *chunk.last().unwrap();

        let mut vals = [0u8; 3];
        for (i, &b) in chunk.iter().enumerate() {
            if b == b'-' {
                if !allow_hyphen {
                    return Err(Error::InvalidCharacter);
                }
            } else if b == b'_' && !allow_underscore {
                return Err(Error::InvalidCharacter);
            }

            let Some(v) = sixbit_value(b) else {
                return Err(Error::InvalidCharacter);
            };
            vals[i] = v;

            if pending_delim_error.is_none() {
                if let Some(comp) = composition {
                    match comp {
                        Composition::Alphanumeric => {}
                        Composition::AlphanumericHyphen => {
                            if b == b'-' {
                                if last_delim == Some(b'-') && !allow_consecutive_hyphens {
                                    pending_delim_error = Some(Error::ConsecutiveHyphens);
                                }
                                last_delim = Some(b'-');
                            } else {
                                last_delim = None;
                            }
                        }
                        Composition::AlphanumericUnderscore => {
                            if b == b'_' {
                                if last_delim == Some(b'_') && !allow_consecutive_underscores {
                                    pending_delim_error = Some(Error::ConsecutiveUnderscores);
                                }
                                last_delim = Some(b'_');
                            } else {
                                last_delim = None;
                            }
                        }
                        Composition::AlphanumericHyphenUnderscore => match b {
                            b'-' | b'_' => {
                                if let Some(prev) = last_delim {
                                    if prev == b {
                                        if b == b'-' && !allow_consecutive_hyphens {
                                            pending_delim_error = Some(Error::ConsecutiveHyphens);
                                        }
                                        if b == b'_' && !allow_consecutive_underscores {
                                            pending_delim_error =
                                                Some(Error::ConsecutiveUnderscores);
                                        }
                                    } else if !delimiter_rules.allow_adjacent_hyphen_underscore() {
                                        pending_delim_error = Some(Error::AdjacentHyphenUnderscore);
                                    }
                                }
                                last_delim = Some(b);
                            }
                            _ => {
                                last_delim = None;
                            }
                        },
                    }
                }
            } else if let Some(comp) = composition {
                match comp {
                    Composition::AlphanumericHyphen => {
                        last_delim = if b == b'-' { Some(b'-') } else { None };
                    }
                    Composition::AlphanumericUnderscore => {
                        last_delim = if b == b'_' { Some(b'_') } else { None };
                    }
                    Composition::AlphanumericHyphenUnderscore => {
                        last_delim = match b {
                            b'-' | b'_' => Some(b),
                            _ => None,
                        };
                    }
                    Composition::Alphanumeric => {}
                }
            }
        }

        let byte_idx = full_chunks * 3;
        match chunk.len() {
            3 => {
                let a = vals[0];
                let b = vals[1];
                let c = vals[2];
                bytes[byte_idx] = (a << 2) | (b >> 4);
                bytes[byte_idx + 1] = ((b & MASK_FOUR_BITS) << 4) | (c >> 2);
                bytes[byte_idx + 2] = (c & MASK_TWO_BITS) << 6;
            }
            2 => {
                let a = vals[0];
                let b = vals[1];
                bytes[byte_idx] = (a << 2) | (b >> 4);
                bytes[byte_idx + 1] = (b & MASK_FOUR_BITS) << 4;
            }
            1 => {
                let a = vals[0];
                bytes[byte_idx] = a << 2;
            }
            _ => unreachable!(),
        }
    } else if len > 0 {
        first_byte = input[0];
        last_byte = input[len - 1];
    }

    if let Some(err) = pending_delim_error {
        return Err(err);
    }

    if len > 0 {
        if (first_byte == b'-' && !delimiter_rules.allow_leading_hyphens())
            || (last_byte == b'-' && !delimiter_rules.allow_trailing_hyphens())
        {
            return Err(Error::LeadingTrailingHyphen);
        }
        if (first_byte == b'_' && !delimiter_rules.allow_leading_underscores())
            || (last_byte == b'_' && !delimiter_rules.allow_trailing_underscores())
        {
            return Err(Error::LeadingTrailingUnderscore);
        }
    }

    Ok(bytes)
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
    use hexaurl_validate::Error;

    #[test]
    fn test_encode_valid_input() {
        let input = "hello";
        let encoded = encode::<16>(input).unwrap();
        assert_eq!(encoded.len(), 16);
    }

    #[test]
    fn test_encode_with_config() {
        let input = "world";
        let config = Config::<16>::default();
        let encoded = encode_with_config::<16>(input, &config).unwrap();
        assert_eq!(encoded.len(), 16);
    }

    #[test]
    fn test_encode_quick_valid() {
        let input = "test";
        let encoded_opt = encode_quick::<16>(input);
        assert!(encoded_opt.is_ok());
        let encoded = encoded_opt.unwrap();
        assert_eq!(encoded.len(), 16);
    }

    #[test]
    fn test_encode_quick_invalid() {
        // Using '😃' which is not in the allowed SIXBIT range.
        let input = "invalid😃";
        let encoded_opt = encode_quick::<16>(input);
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
        let config = Config::<12>::default();
        let encoded = encode_with_config::<12>(input, &config).unwrap();
        assert_eq!(encoded.len(), 12);
    }

    #[test]
    fn test_encode_quick_non16() {
        let input = "abc";
        let encoded_opt = encode_quick::<9>(input);
        assert!(encoded_opt.is_ok());
        let encoded = encoded_opt.unwrap();
        assert_eq!(encoded.len(), 9);
    }

    #[test]
    fn test_encode_delimiter_error_precedence() {
        let input = "a-_😃";
        let config = Config::<16>::builder()
            .composition(hexaurl_config::Composition::AlphanumericHyphenUnderscore)
            .build()
            .unwrap();
        let res = encode_with_config::<16>(input, &config);
        assert_eq!(res, Err(Error::InvalidCharacter));
    }

    #[test]
    fn test_encode_consecutive_hyphens_error() {
        let input = "--a";
        let config = Config::<16>::default();
        let res = encode_with_config::<16>(input, &config);
        assert_eq!(res, Err(Error::ConsecutiveHyphens));
    }
}
