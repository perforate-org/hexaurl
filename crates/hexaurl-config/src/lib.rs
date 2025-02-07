#![doc = include_str!("../README.md")]

pub mod validate;
use validate::ValidationConfig;

/// Configuration for generating or validating a hexaurl.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Config {
    case: CaseType,
    validation: ValidationConfig,
}

impl Config {
    /// Creates a new configuration.
    pub fn new(case: CaseType, validation: ValidationConfig) -> Self {
        Self {
            case,
            validation,
        }
    }

    /// Creates a new builder for the configuration.
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Returns the letter case type.
    pub fn case(&self) -> CaseType {
        self.case
    }

    /// Returns the validation configuration.
    pub fn validation(&self) -> ValidationConfig {
        self.validation
    }
}

/// Builder for [`Config`].
#[derive(Default)]
pub struct ConfigBuilder {
    case: Option<CaseType>,
    validation: Option<ValidationConfig>,
}

impl ConfigBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the case type.
    pub fn case(mut self, case: CaseType) -> Self {
        self.case = Some(case);
        self
    }

    /// Sets the validation configuration.
    pub fn validation(mut self, validation: ValidationConfig) -> Self {
        self.validation = Some(validation);
        self
    }

    /// Builds the [`Config`]. Missing values are set to their defaults.
    pub fn build(self) -> Config {
        Config {
            case: self.case.unwrap_or_default(),
            validation: self.validation.unwrap_or_default(),
        }
    }
}

/// The case type used when representing the hexaurl.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub enum CaseType {
    /// Lower case letters.
    #[default]
    Lower,
    /// Upper case letters.
    Upper,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validate::{DelimiterRulesBuilder, IdentifierComposition, ValidationConfigBuilder};

    #[test]
    fn test_config_builder_defaults() {
        // Build an empty config: all values fall-back to defaults.
        let config = ConfigBuilder::new().build();
        assert_eq!(config.case(), CaseType::Lower);

        // The default validation config uses a default identifier composition of AlphanumericHyphen.
        assert_eq!(config.validation().identifier(), IdentifierComposition::AlphanumericHyphen);
    }

    #[test]
    fn test_config_builder_custom_values() {
        let delimiter = DelimiterRulesBuilder::new()
            .allow_leading_trailing_hyphens(true)
            .allow_consecutive_underscores(true)
            .build();

        let validation = ValidationConfigBuilder::new()
            .min_length(Some(5))
            .max_length(Some(10))
            .identifier(IdentifierComposition::AlphanumericUnderscore)
            .delimiter(Some(delimiter))
            .build();

        let config = ConfigBuilder::new()
            .case(CaseType::Upper)
            .validation(validation)
            .build();

        assert_eq!(config.case(), CaseType::Upper);
        assert_eq!(config.validation().min_length(), Some(5));
        assert_eq!(config.validation().max_length(), Some(10));
        assert_eq!(config.validation().identifier(), IdentifierComposition::AlphanumericUnderscore);
        // Check one of the delimiter rules
        assert!(config.validation().delimiter().unwrap().allow_leading_trailing_hyphens());
    }
}
