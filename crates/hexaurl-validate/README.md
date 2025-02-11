# hexaurl-validate

Validation functionality for the HexaURL format.

## Features

This crate provides:

- Character set validation
- Length constraints
- Delimiter rules validation
- Configuration system
- Error types and handling

## Usage

```rust
use hexaurl_validate::{validate, validate_with_config, Error};
use hexaurl_config::{Config, IdentifierComposition};

// Basic validation with default config
let result = validate::<16>("Hello-World");
assert!(result.is_ok());

// Custom validation configuration
let config = Config::builder()
    .min_length(Some(5))
    .identifier(IdentifierComposition::AlphanumericHyphen)
    .build();

let result = validate_with_config::<16>("Hello-World", config);
```

## Validation Rules

### Character Sets

Four different character set modes are available:

```rust
use hexaurl_config::IdentifierComposition;

// Alphanumeric only (A-Z, 0-9)
IdentifierComposition::Alphanumeric;

// Alphanumeric + hyphen
IdentifierComposition::AlphanumericHyphen;

// Alphanumeric + underscore
IdentifierComposition::AlphanumericUnderscore;

// Alphanumeric + both delimiters
IdentifierComposition::AlphanumericHyphenUnderscore;
```

### Length Constraints

```rust
use hexaurl_config::Config;

let config = Config::builder()
    .min_length(Some(5))    // Minimum 5 characters
    .max_length(Some(20))   // Maximum 20 characters
    .build();
```

### Delimiter Rules

Control delimiter behavior:

- Leading/trailing delimiters
- Consecutive delimiters
- Adjacent different delimiters

```rust
use hexaurl_config::{Config, DelimiterRules};

let delimiter_config = DelimiterRules::builder()
    .allow_consecutive_hyphens(false)
    .allow_leading_trailing_hyphens(false)
    .allow_adjacent_hyphen_underscore(false)
    .build();

let config = Config::builder()
    .delimiter(Some(delimiter_config))
    .build();
```

## Error Handling

The crate provides detailed error types:

```rust
pub enum Error {
    InvalidCharacter,
    StringTooShort(u8),
    StringTooLong(u8),
    ConsecutiveHyphens,
    ConsecutiveUnderscores,
    LeadingTrailingHyphen,
    LeadingTrailingUnderscore,
    AdjacentHyphenUnderscore,
}
```

## Performance

The validation is optimized for performance:

- Single-pass validation
- No string allocations
- Early return on validation failures
- Inline character validation

See the root README.md for complete documentation.
