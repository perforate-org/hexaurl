/// Errors that can occur when working with HexaURL
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    // Common errors
    /// The input string length is too long
    #[error("String is too long: maximum length is {0} characters")]
    StringTooLong(usize),

    /// The input string length is too short
    #[error("String is too short: minimum length is {0} characters")]
    StringTooShort(usize),

    /// The input bytes length is too long
    #[error("Bytes exceed maximum length: {0} bytes")]
    BytesTooLong(usize),

    /// The input bytes length is too short
    #[error("Bytes below minimum length: {0} bytes")]
    BytesTooShort(usize),

    /// The input includes characters invalid for this type of HexaURL encoding
    #[error("Invalid character in this type of HexaURL")]
    InvalidCharacter,

    /// The input includes bytes invalid for this type of HexaURL encoding
    #[error("Invalid byte in this type of HexaURL")]
    InvalidByte,

    /// The input length is invalid for this type of HexaURL encoding
    #[error("Invalid length for this type of HexaURL")]
    InvalidLength,

    /// The input configuration is invalid
    #[error("Maximum length {0} cannot be less than minimum length {1}")]
    InvalidConfig(usize, usize),

    // Errors limited by configuration
    /// The input includes hyphens at the start or end (not allowed by configuration)
    #[error("Hyphens cannot start or end this type of HexaURL")]
    LeadingTrailingHyphen,

    /// The input includes underscores at the start or end (not allowed by configuration)
    #[error("Underscores cannot start or end this type of HexaURL")]
    LeadingTrailingUnderscore,

    /// The input includes consecutive hyphens (not allowed by configuration)
    #[error("This type of HexaURL cannot include consecutive hyphens")]
    ConsecutiveHyphens,

    /// The input includes consecutive underscores (not allowed by configuration)
    #[error("This type of HexaURL cannot include consecutive underscores")]
    ConsecutiveUnderscores,

    /// The input includes adjacent hyphens and underscores (not allowed by configuration)
    #[error("This type of HexaURL cannot include adjacent hyphens and underscores")]
    AdjacentHyphenUnderscore,
}
