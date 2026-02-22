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
    .build()
    .unwrap();
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
    .build()
    .unwrap();
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

`Config::default()`:

- Minimum length: `Some(3)`
- Maximum length: `None` (effective max is enforced by the target byte-size at validation/encoding time)
- Composition: `AlphanumericHyphen`
- Delimiter rules: `None` (validator uses `DelimiterRules::default()`)

`Config::minimal()`:

- Minimum length: `None`
- Maximum length: `None`
- Composition: `AlphanumericHyphenUnderscore`
- Delimiter rules: `Some(DelimiterRules::all_allowed())`

See [the root README.md](https://github.com/perforate-org/hexaurl#readme) for complete documentation.
