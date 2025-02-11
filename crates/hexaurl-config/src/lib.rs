#![doc = include_str!("../README.md")]

/// Configuration for validation rules.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Config {
    min_length: Option<usize>,
    max_length: Option<usize>,
    identifier: IdentifierComposition,
    delimiter: Option<DelimiterRules>,
}

impl Config {
    /// Constructs a new validation configuration.
    pub fn new(
        min_length: Option<usize>,
        max_length: Option<usize>,
        identifier: IdentifierComposition,
        delimiter: Option<DelimiterRules>,
    ) -> Self {
        Self {
            min_length,
            max_length,
            identifier,
            delimiter,
        }
    }

    /// Constructs a minimal validation configuration.
    pub fn minimal() -> Self {
        Self {
            min_length: None,
            max_length: None,
            identifier: IdentifierComposition::AlphanumericHyphenUnderscore,
            delimiter: Some(DelimiterRules::all_allowed()),
        }
    }

    /// Creates a new builder for validation config.
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Returns the minimum allowed length.
    pub fn min_length(&self) -> Option<usize> {
        self.min_length
    }

    /// Returns the maximum allowed length.
    pub fn max_length(&self) -> Option<usize> {
        self.max_length
    }

    /// Returns the identifier composition rule.
    pub fn identifier(&self) -> IdentifierComposition {
        self.identifier
    }

    /// Returns the delimiter rules, if any.
    pub fn delimiter(&self) -> Option<DelimiterRules> {
        self.delimiter
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            min_length: Some(3),
            max_length: None,
            identifier: IdentifierComposition::default(),
            delimiter: None,
        }
    }
}

/// Builder for [`Config`].
#[derive(Default)]
pub struct ConfigBuilder {
    min_length: Option<usize>,
    max_length: Option<usize>,
    identifier: Option<IdentifierComposition>,
    delimiter: Option<DelimiterRules>,
}

impl ConfigBuilder {
    /// Creates a new builder for validation config.
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
    pub fn identifier(mut self, identifier: IdentifierComposition) -> Self {
        self.identifier = Some(identifier);
        self
    }

    /// Sets the delimiter rules.
    pub fn delimiter(mut self, delimiter: Option<DelimiterRules>) -> Self {
        self.delimiter = delimiter;
        self
    }

    /// Builds the [`ValidationConfig`]. Missing values default to those defined by [`Default`].
    pub fn build(self) -> Config {
        Config {
            min_length: self.min_length.or_else(|| Config::default().min_length()),
            max_length: self.max_length.or_else(|| Config::default().max_length()),
            identifier: self.identifier.unwrap_or_default(),
            delimiter: self.delimiter,
        }
    }
}

/// Valid options for identifier composition.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub enum IdentifierComposition {
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
            allow_leading_trailing_underscores: self.allow_leading_trailing_underscores.unwrap_or(false),
            allow_consecutive_hyphens: self.allow_consecutive_hyphens.unwrap_or(false),
            allow_consecutive_underscores: self.allow_consecutive_underscores.unwrap_or(false),
            allow_adjacent_hyphen_underscore: self.allow_adjacent_hyphen_underscore.unwrap_or(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_config_builder_custom_values() {
        let delimiter = DelimiterRulesBuilder::new()
            .allow_leading_trailing_underscores(true)
            .allow_consecutive_hyphens(true)
            .build();

        let vc = Config::builder()
            .min_length(Some(4))
            .max_length(Some(12))
            .identifier(IdentifierComposition::AlphanumericHyphenUnderscore)
            .delimiter(Some(delimiter))
            .build();

        assert_eq!(vc.min_length(), Some(4));
        assert_eq!(vc.max_length(), Some(12));
        assert_eq!(vc.identifier(), IdentifierComposition::AlphanumericHyphenUnderscore);
        assert!(vc.delimiter().unwrap().allow_consecutive_hyphens());
        assert!(vc.delimiter().unwrap().allow_leading_trailing_underscores());
    }
}
