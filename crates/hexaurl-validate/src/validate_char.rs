use crate::Error;

/// Validate that the given ASCII code is alphanumeric, hyphen or underscore.
#[inline(always)]
pub const fn validate_alphanumeric_with_hyphen_or_underscore(code: u8) -> Result<(), Error> {
    match code {
        45 | 48..=57 | 65..=90 | 95 | 97..=122 => Ok(()),
        _ => Err(Error::InvalidCharacter),
    }
}

/// Validate that the given ASCII code is alphanumeric or underscore.
#[inline(always)]
pub const fn validate_alphanumeric_with_underscore(code: u8) -> Result<(), Error> {
    match code {
        48..=57 | 65..=90 | 95 | 97..=122 => Ok(()),
        _ => Err(Error::InvalidCharacter),
    }
}

/// Validate that the given ASCII code is alphanumeric or hyphen.
#[inline(always)]
pub const fn validate_alphanumeric_with_hyphen(code: u8) -> Result<(), Error> {
    match code {
        45 | 48..=57 | 65..=90 | 97..=122 => Ok(()),
        _ => Err(Error::InvalidCharacter),
    }
}

/// Validate that the given ASCII code is alphanumeric.
#[inline(always)]
pub const fn validate_alphanumeric(code: u8) -> Result<(), Error> {
    match code {
        48..=57 | 65..=90 | 97..=122 => Ok(()),
        _ => Err(Error::InvalidCharacter),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for validate_alphanumeric: numbers, uppercase and lowercase letters are valid.
    #[test]
    fn test_validate_alphanumeric() {
        // Valid characters: numbers, letters (both uppercase and lowercase)
        assert_eq!(validate_alphanumeric(b'0'), Ok(()));
        assert_eq!(validate_alphanumeric(b'9'), Ok(()));
        assert_eq!(validate_alphanumeric(b'A'), Ok(()));
        assert_eq!(validate_alphanumeric(b'Z'), Ok(()));
        assert_eq!(validate_alphanumeric(b'a'), Ok(()));
        assert_eq!(validate_alphanumeric(b'z'), Ok(()));

        // Invalid characters: hyphen, underscore, space, etc.
        assert_eq!(validate_alphanumeric(b'-'), Err(Error::InvalidCharacter));
        assert_eq!(validate_alphanumeric(b'_'), Err(Error::InvalidCharacter));
        assert_eq!(validate_alphanumeric(b' '), Err(Error::InvalidCharacter));
    }

    // Tests for validate_alphanumeric_with_hyphen: numbers, letters and hyphen are valid.
    #[test]
    fn test_validate_alphanumeric_with_hyphen() {
        // Valid characters: numbers, letters (both uppercase and lowercase) and hyphen ('-')
        assert_eq!(validate_alphanumeric_with_hyphen(b'0'), Ok(()));
        assert_eq!(validate_alphanumeric_with_hyphen(b'9'), Ok(()));
        assert_eq!(validate_alphanumeric_with_hyphen(b'A'), Ok(()));
        assert_eq!(validate_alphanumeric_with_hyphen(b'Z'), Ok(()));
        assert_eq!(validate_alphanumeric_with_hyphen(b'a'), Ok(()));
        assert_eq!(validate_alphanumeric_with_hyphen(b'z'), Ok(()));
        assert_eq!(validate_alphanumeric_with_hyphen(b'-'), Ok(()));

        // Invalid characters: underscore, space, etc.
        assert_eq!(validate_alphanumeric_with_hyphen(b'_'), Err(Error::InvalidCharacter));
        assert_eq!(validate_alphanumeric_with_hyphen(b' '), Err(Error::InvalidCharacter));
    }

    // Tests for validate_alphanumeric_with_underscore: numbers, letters and underscore are valid.
    #[test]
    fn test_validate_alphanumeric_with_underscore() {
        // Valid characters: numbers, letters (both uppercase and lowercase) and underscore ('_')
        assert_eq!(validate_alphanumeric_with_underscore(b'0'), Ok(()));
        assert_eq!(validate_alphanumeric_with_underscore(b'9'), Ok(()));
        assert_eq!(validate_alphanumeric_with_underscore(b'A'), Ok(()));
        assert_eq!(validate_alphanumeric_with_underscore(b'Z'), Ok(()));
        assert_eq!(validate_alphanumeric_with_underscore(b'a'), Ok(()));
        assert_eq!(validate_alphanumeric_with_underscore(b'z'), Ok(()));
        assert_eq!(validate_alphanumeric_with_underscore(b'_'), Ok(()));

        // Invalid characters: hyphen, space, etc.
        assert_eq!(validate_alphanumeric_with_underscore(b'-'), Err(Error::InvalidCharacter));
        assert_eq!(validate_alphanumeric_with_underscore(b' '), Err(Error::InvalidCharacter));
    }

    // Tests for validate_alphanumeric_with_hyphen_or_underscore: numbers, letters, hyphen and underscore are valid.
    #[test]
    fn test_validate_alphanumeric_with_hyphen_or_underscore() {
        // Valid characters: numbers, letters (both uppercase and lowercase), hyphen ('-') and underscore ('_')
        assert_eq!(validate_alphanumeric_with_hyphen_or_underscore(b'0'), Ok(()));
        assert_eq!(validate_alphanumeric_with_hyphen_or_underscore(b'9'), Ok(()));
        assert_eq!(validate_alphanumeric_with_hyphen_or_underscore(b'A'), Ok(()));
        assert_eq!(validate_alphanumeric_with_hyphen_or_underscore(b'Z'), Ok(()));
        assert_eq!(validate_alphanumeric_with_hyphen_or_underscore(b'a'), Ok(()));
        assert_eq!(validate_alphanumeric_with_hyphen_or_underscore(b'z'), Ok(()));
        assert_eq!(validate_alphanumeric_with_hyphen_or_underscore(b'-'), Ok(()));
        assert_eq!(validate_alphanumeric_with_hyphen_or_underscore(b'_'), Ok(()));

        // Invalid characters: space, etc.
        assert_eq!(validate_alphanumeric_with_hyphen_or_underscore(b' '), Err(Error::InvalidCharacter));
    }
}
