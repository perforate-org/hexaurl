# hexaurl-config

Configuration types and builders for the HexaURL format.

## Features

This crate provides:

- Configuration types for validation rules
- Builder patterns for easy configuration
- Reusable validation settings
- Default configurations

## Usage

```rust
use hexaurl_config::{
    Config,
    Composition,
    DelimiterRules,
};

// Create a complete configuration
let config = Config::builder()
    .min_length(Some(5))
    .max_length(Some(20))
    .composition(Composition::AlphanumericHyphen)
    .delimiter(Some(
        DelimiterRules::builder()
            .allow_consecutive_hyphens(false)
            .allow_leading_trailing_hyphens(false)
            .build()
    ))
    .build();
```

## Configuration Components

### Validation Config

Controls the overall validation behavior:

- String length constraints
- Character set composition
- Delimiter rules

```rust
use hexaurl_config::{
    Config,
    Composition,
};

let config = Config::builder()
    .min_length(Some(5))
    .max_length(Some(20))
    .composition(Composition::Alphanumeric)
    .build();
```

### Identifier Composition

Defines allowed character sets:

- `Alphanumeric`: A-Z, 0-9
- `AlphanumericHyphen`: A-Z, 0-9, -
- `AlphanumericUnderscore`: A-Z, 0-9, \_
- `AlphanumericHyphenUnderscore`: A-Z, 0-9, -, \_

### Delimiter Configuration

Controls delimiter behavior:

```rust
use hexaurl_config::DelimiterRules;

let delimiter_config = DelimiterRules::builder()
    .allow_consecutive_hyphens(false)
    .allow_consecutive_underscores(false)
    .allow_leading_trailing_hyphens(false)
    .allow_leading_trailing_underscores(false)
    .allow_adjacent_hyphen_underscore(false)
    .build();
```

## Default Values

The default configuration:

- No minimum length
- Maximum length of the encoded HexaURL string representation
- Alphanumeric with hyphens and underscores
- No consecutive delimiters
- No leading/trailing delimiters
- No adjacent different delimiters

See [the root README.md](https://github.com/perforate-org/hexaurl#readme) for complete documentation.
