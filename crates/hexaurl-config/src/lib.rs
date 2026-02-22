#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use core::fmt;

/// Error type for invalid configuration values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    /// Minimum length is greater than maximum length in builder input.
    InvalidLengthRange {
        /// Provided minimum length.
        min: usize,
        /// Provided maximum length.
        max: usize,
    },
    /// Minimum length is greater than effective max for the target capacity.
    InvalidCompiledLengthRange {
        /// Provided minimum length.
        min: usize,
        /// Effective maximum length.
        max: usize,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLengthRange { min, max } => {
                write!(
                    f,
                    "Minimum length {min} cannot be greater than maximum length {max}"
                )
            }
            Self::InvalidCompiledLengthRange { min, max } => {
                write!(
                    f,
                    "Minimum length {min} cannot be greater than compiled maximum length {max}"
                )
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#[inline(always)]
const fn calc_str_len(n: usize) -> usize {
    n * 4 / 3
}

#[inline]
fn validate_length_range(
    min_length: Option<usize>,
    max_length: Option<usize>,
) -> Result<(), ConfigError> {
    if let (Some(min), Some(max)) = (min_length, max_length) {
        if min > max {
            return Err(ConfigError::InvalidLengthRange { min, max });
        }
    }
    Ok(())
}

/// Precompiled validation configuration for a specific HexaURL byte size `N`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Config<const N: usize> {
    min_length: Option<usize>,
    effective_max: usize,
    composition: Composition,
    delimiter_rules: DelimiterRules,
    allow_hyphen: bool,
    allow_underscore: bool,
}

impl<const N: usize> Config<N> {
    /// Creates a builder that compiles directly into [`Config`].
    pub fn builder() -> ConfigBuilder<N> {
        ConfigBuilder::default()
    }

    /// Creates a minimally restricted compiled config.
    pub fn minimal() -> Self {
        Self::builder()
            .min_length(None)
            .max_length(None)
            .composition(Composition::AlphanumericHyphenUnderscore)
            .delimiter(Some(DelimiterRules::all_allowed()))
            .build()
            .expect("minimal config is valid")
    }

    /// Returns the minimum allowed length.
    pub fn min_length(&self) -> Option<usize> {
        self.min_length
    }

    /// Returns the effective maximum allowed length.
    pub fn effective_max(&self) -> usize {
        self.effective_max
    }

    /// Returns the identifier composition rule.
    pub fn composition(&self) -> Composition {
        self.composition
    }

    /// Returns the delimiter rules.
    pub fn delimiter_rules(&self) -> DelimiterRules {
        self.delimiter_rules
    }

    /// Whether hyphen is allowed by composition.
    pub fn allow_hyphen(&self) -> bool {
        self.allow_hyphen
    }

    /// Whether underscore is allowed by composition.
    pub fn allow_underscore(&self) -> bool {
        self.allow_underscore
    }
}

impl<const N: usize> Default for Config<N> {
    fn default() -> Self {
        ConfigBuilder::default()
            .build()
            .expect("default config is always valid")
    }
}

/// Builder for compiled [`Config`].
pub struct ConfigBuilder<const N: usize> {
    min_length: Option<usize>,
    max_length: Option<usize>,
    composition: Composition,
    delimiter: Option<DelimiterRules>,
}

impl<const N: usize> Default for ConfigBuilder<N> {
    fn default() -> Self {
        Self {
            min_length: Some(3),
            max_length: None,
            composition: Composition::default(),
            delimiter: None,
        }
    }
}

impl<const N: usize> ConfigBuilder<N> {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the minimum allowed length.
    pub fn min_length(mut self, min: Option<usize>) -> Self {
        self.min_length = min;
        self
    }

    /// Sets the maximum allowed length.
    pub fn max_length(mut self, max: Option<usize>) -> Self {
        self.max_length = max;
        self
    }

    /// Sets the identifier composition.
    pub fn composition(mut self, composition: Composition) -> Self {
        self.composition = composition;
        self
    }

    /// Sets the delimiter rules.
    pub fn delimiter(mut self, delimiter: Option<DelimiterRules>) -> Self {
        self.delimiter = delimiter;
        self
    }

    /// Builds a compiled [`Config`].
    pub fn build(self) -> Result<Config<N>, ConfigError> {
        validate_length_range(self.min_length, self.max_length)?;

        let capacity_max = calc_str_len(N);
        let effective_max = self
            .max_length
            .map(|max| core::cmp::min(max, capacity_max))
            .unwrap_or(capacity_max);

        if let Some(min) = self.min_length {
            if min > effective_max {
                return Err(ConfigError::InvalidCompiledLengthRange {
                    min,
                    max: effective_max,
                });
            }
        }

        let delimiter_rules = self.delimiter.unwrap_or_default();
        let (allow_hyphen, allow_underscore) = match self.composition {
            Composition::Alphanumeric => (false, false),
            Composition::AlphanumericHyphen => (true, false),
            Composition::AlphanumericUnderscore => (false, true),
            Composition::AlphanumericHyphenUnderscore => (true, true),
        };

        Ok(Config {
            min_length: self.min_length,
            effective_max,
            composition: self.composition,
            delimiter_rules,
            allow_hyphen,
            allow_underscore,
        })
    }
}

/// Valid options for identifier composition.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub enum Composition {
    /// Letters and digits.
    Alphanumeric,
    /// Letters, digits and hyphen.
    #[default]
    AlphanumericHyphen,
    /// Letters, digits and underscore.
    AlphanumericUnderscore,
    /// Letters, digits, hyphen and underscore.
    AlphanumericHyphenUnderscore,
}

/// Rules for allowed delimiters.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct DelimiterRules {
    allow_leading_trailing_hyphens: bool,
    allow_leading_trailing_underscores: bool,
    allow_consecutive_hyphens: bool,
    allow_consecutive_underscores: bool,
    allow_adjacent_hyphen_underscore: bool,
}

impl DelimiterRules {
    /// Creates a new set of delimiter rules.
    pub fn new(
        allow_leading_trailing_hyphens: bool,
        allow_leading_trailing_underscores: bool,
        allow_consecutive_hyphens: bool,
        allow_consecutive_underscores: bool,
        allow_adjacent_hyphen_underscore: bool,
    ) -> Self {
        Self {
            allow_leading_trailing_hyphens,
            allow_leading_trailing_underscores,
            allow_consecutive_hyphens,
            allow_consecutive_underscores,
            allow_adjacent_hyphen_underscore,
        }
    }

    /// Creates a new set of delimiter rules with all rules allowed.
    pub fn all_allowed() -> Self {
        Self {
            allow_leading_trailing_hyphens: true,
            allow_leading_trailing_underscores: true,
            allow_consecutive_hyphens: true,
            allow_consecutive_underscores: true,
            allow_adjacent_hyphen_underscore: true,
        }
    }

    /// Creates a new builder for delimiter rules.
    pub fn builder() -> DelimiterRulesBuilder {
        DelimiterRulesBuilder::new()
    }

    /// Whether leading and trailing hyphens are allowed.
    pub fn allow_leading_trailing_hyphens(&self) -> bool {
        self.allow_leading_trailing_hyphens
    }

    /// Whether leading and trailing underscores are allowed.
    pub fn allow_leading_trailing_underscores(&self) -> bool {
        self.allow_leading_trailing_underscores
    }

    /// Whether consecutive hyphens are allowed.
    pub fn allow_consecutive_hyphens(&self) -> bool {
        self.allow_consecutive_hyphens
    }

    /// Whether consecutive underscores are allowed.
    pub fn allow_consecutive_underscores(&self) -> bool {
        self.allow_consecutive_underscores
    }

    /// Whether a hyphen and an underscore can be adjacent.
    pub fn allow_adjacent_hyphen_underscore(&self) -> bool {
        self.allow_adjacent_hyphen_underscore
    }
}

/// Builder for [`DelimiterRules`].
#[derive(Default)]
pub struct DelimiterRulesBuilder {
    allow_leading_trailing_hyphens: Option<bool>,
    allow_leading_trailing_underscores: Option<bool>,
    allow_consecutive_hyphens: Option<bool>,
    allow_consecutive_underscores: Option<bool>,
    allow_adjacent_hyphen_underscore: Option<bool>,
}

impl DelimiterRulesBuilder {
    /// Creates a new builder for delimiter rules.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether leading and trailing hyphens are allowed.
    pub fn allow_leading_trailing_hyphens(mut self, allow: bool) -> Self {
        self.allow_leading_trailing_hyphens = Some(allow);
        self
    }

    /// Sets whether leading and trailing underscores are allowed.
    pub fn allow_leading_trailing_underscores(mut self, allow: bool) -> Self {
        self.allow_leading_trailing_underscores = Some(allow);
        self
    }

    /// Sets whether consecutive hyphens are allowed.
    pub fn allow_consecutive_hyphens(mut self, allow: bool) -> Self {
        self.allow_consecutive_hyphens = Some(allow);
        self
    }

    /// Sets whether consecutive underscores are allowed.
    pub fn allow_consecutive_underscores(mut self, allow: bool) -> Self {
        self.allow_consecutive_underscores = Some(allow);
        self
    }

    /// Sets whether adjacent hyphen and underscore are allowed.
    pub fn allow_adjacent_hyphen_underscore(mut self, allow: bool) -> Self {
        self.allow_adjacent_hyphen_underscore = Some(allow);
        self
    }

    /// Builds the [`DelimiterRules`] object.
    ///
    /// Missing rules default to false.
    pub fn build(self) -> DelimiterRules {
        DelimiterRules {
            allow_leading_trailing_hyphens: self.allow_leading_trailing_hyphens.unwrap_or(false),
            allow_leading_trailing_underscores: self
                .allow_leading_trailing_underscores
                .unwrap_or(false),
            allow_consecutive_hyphens: self.allow_consecutive_hyphens.unwrap_or(false),
            allow_consecutive_underscores: self.allow_consecutive_underscores.unwrap_or(false),
            allow_adjacent_hyphen_underscore: self
                .allow_adjacent_hyphen_underscore
                .unwrap_or(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder_new() {
        let builder = ConfigBuilder::<16>::new();
        assert_eq!(builder.min_length, Some(3));
        assert_eq!(builder.max_length, None);
        assert_eq!(builder.composition, Composition::AlphanumericHyphen);
        assert_eq!(builder.delimiter, None);
    }

    #[test]
    fn test_config_builder_custom_values() {
        let delimiter = DelimiterRulesBuilder::new()
            .allow_leading_trailing_underscores(true)
            .allow_consecutive_hyphens(true)
            .build();

        let config = Config::<16>::builder()
            .min_length(Some(4))
            .max_length(Some(12))
            .composition(Composition::AlphanumericHyphenUnderscore)
            .delimiter(Some(delimiter))
            .build()
            .unwrap();

        assert_eq!(config.min_length(), Some(4));
        assert_eq!(config.effective_max(), 12);
        assert_eq!(
            config.composition(),
            Composition::AlphanumericHyphenUnderscore
        );
        assert!(config.delimiter_rules().allow_consecutive_hyphens());
        assert!(config
            .delimiter_rules()
            .allow_leading_trailing_underscores());
        assert!(config.allow_hyphen());
        assert!(config.allow_underscore());
    }

    #[test]
    fn test_config_minimal() {
        let config = Config::<16>::minimal();
        assert_eq!(config.min_length(), None);
        assert_eq!(
            config.composition(),
            Composition::AlphanumericHyphenUnderscore
        );
        assert!(config.delimiter_rules().allow_adjacent_hyphen_underscore());
    }

    #[test]
    fn test_delimiter_rules_new() {
        let rules = DelimiterRules::new(true, false, true, false, true);
        assert!(rules.allow_leading_trailing_hyphens());
        assert!(!rules.allow_leading_trailing_underscores());
        assert!(rules.allow_consecutive_hyphens());
        assert!(!rules.allow_consecutive_underscores());
        assert!(rules.allow_adjacent_hyphen_underscore());
    }

    #[test]
    fn test_delimiter_rules_builder() {
        let builder = DelimiterRules::builder();
        assert_eq!(builder.allow_leading_trailing_hyphens, None);
        assert_eq!(builder.allow_leading_trailing_underscores, None);
        assert_eq!(builder.allow_consecutive_hyphens, None);
        assert_eq!(builder.allow_consecutive_underscores, None);
        assert_eq!(builder.allow_adjacent_hyphen_underscore, None);
    }

    #[test]
    fn test_delimiter_rules_all_allowed() {
        let rules = DelimiterRules::all_allowed();
        assert!(rules.allow_leading_trailing_hyphens());
        assert!(rules.allow_leading_trailing_underscores());
        assert!(rules.allow_consecutive_hyphens());
        assert!(rules.allow_consecutive_underscores());
        assert!(rules.allow_adjacent_hyphen_underscore());
    }

    #[test]
    fn test_delimiter_rules_builder_new() {
        let rules = DelimiterRulesBuilder::new()
            .allow_leading_trailing_hyphens(true)
            .allow_consecutive_underscores(true)
            .allow_adjacent_hyphen_underscore(true)
            .build();

        assert!(rules.allow_leading_trailing_hyphens());
        assert!(!rules.allow_leading_trailing_underscores());
        assert!(!rules.allow_consecutive_hyphens());
        assert!(rules.allow_consecutive_underscores());
        assert!(rules.allow_adjacent_hyphen_underscore());
    }

    #[test]
    fn test_invalid_length_config_builder() {
        let err = Config::<16>::builder()
            .min_length(Some(10))
            .max_length(Some(5))
            .build()
            .unwrap_err();
        assert_eq!(err, ConfigError::InvalidLengthRange { min: 10, max: 5 });
    }

    #[test]
    fn test_invalid_compiled_length() {
        let err = Config::<8>::builder()
            .min_length(Some(20))
            .build()
            .unwrap_err();
        assert_eq!(
            err,
            ConfigError::InvalidCompiledLengthRange { min: 20, max: 10 }
        );
    }
}
