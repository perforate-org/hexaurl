#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use config::{Composition, Config};
pub use hexaurl_config as config;
use std::cmp;

mod error;
#[cfg(not(feature = "char"))]
mod validate_char;
#[cfg(feature = "char")]
#[cfg_attr(docsrs, doc(cfg(feature = "char")))]
pub mod validate_char;

pub use error::Error;

/// Calculates the length of the decoded string based on the number of input bytes.
#[inline(always)]
const fn calc_str_len(n: usize) -> usize {
    n * 4 / 3
}

/// Validates a HexaURL string in a single pass with default configuration.
/// Returns Ok(()) if the string meets all criteria, otherwise returns an Error.
#[inline]
pub fn validate<const N: usize>(input: &str) -> Result<(), Error> {
    validate_with_config::<N>(input, Config::default())
}

/// Validates a HexaURL string in a single pass.
/// Returns Ok(()) if the string meets all criteria, otherwise returns an Error.
#[inline]
pub fn validate_with_config<const N: usize>(input: &str, config: Config) -> Result<(), Error> {
    let len = input.len();

    let effective_max = config
        .max_length()
        .map(|max| cmp::min(max, calc_str_len(N)))
        .unwrap_or(calc_str_len(N));

    // Check minimum length.
    if let Some(min) = config.min_length() {
        if min > effective_max {
            return Err(Error::InvalidConfig(effective_max, min));
        }
        if len < min {
            return Err(Error::StringTooShort(min));
        }
    }
    // Check maximum length.
    if len > effective_max {
        return Err(Error::StringTooLong(effective_max));
    }

    // Retrieve delimiter rules.
    let delimiter_rules = config.delimiter().unwrap_or_default();
    let bytes = input.as_bytes();

    // Process each character in a single pass by converting to uppercase on the fly.
    match config.composition() {
        Composition::Alphanumeric => {
            // Validate each character as uppercase alphanumeric.
            for &b in bytes {
                validate_char::validate_alphanumeric(b)?;
            }
        }
        Composition::AlphanumericHyphen => {
            // Track the last delimiter character for consecutive hyphen checks.
            let mut last_delim: Option<u8> = None;
            for &b in bytes {
                validate_char::validate_alphanumeric_with_hyphen(b)?;
                if b == b'-' {
                    // Check if consecutive hyphens are disallowed.
                    if last_delim == Some(b'-') && !delimiter_rules.allow_consecutive_hyphens() {
                        return Err(Error::ConsecutiveHyphens);
                    }
                    last_delim = Some(b'-');
                } else {
                    last_delim = None;
                }
            }
        }
        Composition::AlphanumericUnderscore => {
            // Track the last delimiter character for consecutive underscore checks.
            let mut last_delim: Option<u8> = None;
            for &b in bytes {
                validate_char::validate_alphanumeric_with_underscore(b)?;
                if b == b'_' {
                    // Check if consecutive underscores are disallowed.
                    if last_delim == Some(b'_') && !delimiter_rules.allow_consecutive_underscores()
                    {
                        return Err(Error::ConsecutiveUnderscores);
                    }
                    last_delim = Some(b'_');
                } else {
                    last_delim = None;
                }
            }
        }
        Composition::AlphanumericHyphenUnderscore => {
            // Track the last delimiter (hyphen or underscore) for consecutive and adjacent delimiter checks.
            let mut last_delim: Option<u8> = None;
            for &b in bytes {
                validate_char::validate_alphanumeric_with_hyphen_or_underscore(b)?;
                match b {
                    b'-' | b'_' => {
                        if let Some(prev) = last_delim {
                            // Check for consecutive identical delimiters.
                            if prev == b {
                                if b == b'-' && !delimiter_rules.allow_consecutive_hyphens() {
                                    return Err(Error::ConsecutiveHyphens);
                                }
                                if b == b'_' && !delimiter_rules.allow_consecutive_underscores() {
                                    return Err(Error::ConsecutiveUnderscores);
                                }
                            }
                            // Check for adjacent different delimiters if disallowed.
                            else if !delimiter_rules.allow_adjacent_hyphen_underscore() {
                                return Err(Error::AdjacentHyphenUnderscore);
                            }
                        }
                        last_delim = Some(b);
                    }
                    _ => {
                        last_delim = None;
                    }
                }
            }
        }
    }

    // Validate leading/trailing delimiter characters without iterating over the string again in uppercase.
    // We compare against the raw input because hyphen and underscore are unaffected by case conversion.
    if !delimiter_rules.allow_leading_trailing_hyphens()
        && (input.starts_with('-') || input.ends_with('-'))
    {
        return Err(Error::LeadingTrailingHyphen);
    }
    if !delimiter_rules.allow_leading_trailing_underscores()
        && (input.starts_with('_') || input.ends_with('_'))
    {
        return Err(Error::LeadingTrailingUnderscore);
    }

    Ok(())
}

/// Validates a string against the minimal configuration.
///
/// This function is optimized for speed by performing minimal checks:
/// - Checks maximum length.
/// - Validates each character as alphanumeric with hyphen or underscore.
///
/// # Const Parameters
/// - `N`: The byte size of HexaURL encoded string.
#[inline]
pub fn validate_minimal_config<const N: usize>(input: &str) -> Result<(), Error> {
    let max = calc_str_len(N);

    // Check maximum length.
    if input.len() > max {
        return Err(Error::StringTooLong(max));
    }
    for &b in input.as_bytes() {
        validate_char::validate_alphanumeric_with_hyphen_or_underscore(b)?;
    }

    Ok(())
}

/// Checks if the input string is safe for HexaURL encoding without risk of panics or conflicts.
///
/// This function is optimized for speed by performing minimal checks:
/// it verifies that the input is fully ASCII and that its length does not exceed the maximum
/// based on the given byte size. It is recommended only for use when retrieving keys from a map,
/// not when inserting new entries.
///
/// # Returns
///
/// Returns `true` if the input string satisfies the minimal safety checks, `false` otherwise.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use hexaurl::HexaUrl;
/// use hexaurl_validate::check_encoding_safe;
///
/// let input = "ABC123";
/// let mut map = HashMap::new();
///
/// // Insert a value into the map using the safe encoding function.
/// let insert_key = HexaUrl::new(input).unwrap();
/// map.insert(insert_key, 42);
///
/// // Retrieve the value from the map using the unsafe encoding function.
/// let res = if check_encoding_safe::<16>(input).is_ok() {
///     unsafe {
///         let get_key = HexaUrl::new_unchecked(input);
///         map.get(&get_key)
///     }
/// } else {
///     None
/// };
///
/// assert_eq!(res, Some(&42));
/// ```
#[inline(always)]
pub const fn check_encoding_safe<const N: usize>(input: &str) -> Result<(), Error> {
    if input.len() <= calc_str_len(N) {
        if input.is_ascii() {
            Ok(())
        } else {
            Err(Error::InvalidCharacter)
        }
    } else {
        Err(Error::StringTooLong(calc_str_len(N)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use error::Error;

    // Test that non-ASCII characters are rejected.
    #[test]
    fn test_non_ascii() {
        let result = validate::<16>("abc\u{00E9}");
        assert_eq!(result, Err(Error::InvalidCharacter));
    }

    // Test that a string shorter than the minimum length is rejected.
    #[test]
    fn test_string_too_short() {
        // Build a ValidationConfig with a minimum length of 5 using the builder pattern.
        let config = Config::builder().min_length(Some(5)).build();
        let result = validate_with_config::<16>("abcd", config);
        assert_eq!(result, Err(Error::StringTooShort(5)));
    }

    // Test that a string longer than the effective maximum is rejected.
    #[test]
    fn test_string_too_long() {
        // For ByteSize::U8x8 the maximum bound is 10.
        // We override it with a max_length of 8 so that effective_max = min(8, 10) = 8.
        let config = Config::builder().max_length(Some(8)).build();
        let result = validate_with_config::<16>("abcdefghi", config);
        assert_eq!(result, Err(Error::StringTooLong(8)));
    }

    // Test valid alphanumeric input when only letters and numbers are allowed.
    #[test]
    fn test_alphanumeric_valid() {
        let config = Config::builder()
            .composition(Composition::Alphanumeric)
            .build();
        let result = validate_with_config::<16>("abc123", config);
        assert!(result.is_ok());
    }

    // Test that a hyphen is rejected for an alphanumeric-only identifier.
    #[test]
    fn test_alphanumeric_invalid_char() {
        let config = Config::builder()
            .composition(Composition::Alphanumeric)
            .build();
        let result = validate_with_config::<16>("ab-c123", config);
        // A hyphen is not an uppercase alphanumeric, so we expect an invalid character error.
        assert_eq!(result, Err(Error::InvalidCharacter));
    }

    // Test valid input when hyphens are explicitly allowed.
    #[test]
    fn test_alphanumeric_hyphen_valid() {
        let config = Config::builder()
            .composition(Composition::AlphanumericHyphen)
            .build();
        let result = validate_with_config::<16>("abc-123", config);
        assert!(result.is_ok());
    }

    // Test that consecutive hyphens are rejected (assuming the delimiter rules disallow them).
    #[test]
    fn test_alphanumeric_hyphen_consecutive() {
        // Using default delimiter rules (which disallow consecutive delimiters by default).
        let config = Config::builder()
            .composition(Composition::AlphanumericHyphen)
            .build();
        let result = validate_with_config::<16>("abc--123", config);
        assert_eq!(result, Err(Error::ConsecutiveHyphens));
    }

    // Test that a leading or trailing hyphen causes an error.
    #[test]
    fn test_leading_trailing_hyphen() {
        // Using the default configuration (or None) which – according to our rules – does not allow
        // leading or trailing hyphens.
        let result = validate::<16>("-abc123");
        assert_eq!(result, Err(Error::LeadingTrailingHyphen));

        let result2 = validate::<16>("abc123-");
        assert_eq!(result2, Err(Error::LeadingTrailingHyphen));
    }

    // Test valid input when underscores are allowed.
    #[test]
    fn test_alphanumeric_underscore_valid() {
        let config = Config::builder()
            .composition(Composition::AlphanumericUnderscore)
            .build();
        let result = validate_with_config::<16>("abc_123", config);
        assert!(result.is_ok());
    }

    // Test that consecutive underscores are rejected.
    #[test]
    fn test_alphanumeric_underscore_consecutive() {
        let config = Config::builder()
            .composition(Composition::AlphanumericUnderscore)
            .build();
        let result = validate_with_config::<16>("abc__123", config);
        assert_eq!(result, Err(Error::ConsecutiveUnderscores));
    }

    // Test that a leading or trailing underscore causes an error.
    #[test]
    fn test_leading_trailing_underscore() {
        let config = Config::builder()
            .composition(Composition::AlphanumericUnderscore)
            .build();
        let result = validate_with_config::<16>("_abc123", config);
        assert_eq!(result, Err(Error::LeadingTrailingUnderscore));

        let result2 = validate_with_config::<16>("abc123_", config);
        assert_eq!(result2, Err(Error::LeadingTrailingUnderscore));
    }

    // Test that adjacent different delimiters (hyphen and underscore) are rejected.
    #[test]
    fn test_alphanumeric_hyphen_underscore_adjacent() {
        let config = Config::builder()
            .composition(Composition::AlphanumericHyphenUnderscore)
            .build();
        // Using an input where a hyphen and underscore are adjacent.
        let result = validate_with_config::<16>("abc-_123", config);
        assert_eq!(result, Err(Error::AdjacentHyphenUnderscore));
    }
}
