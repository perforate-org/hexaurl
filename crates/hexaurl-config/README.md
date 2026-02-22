# hexaurl-config

Configuration types for the HexaURL format.

## Features

This crate provides:

- `Config<const N: usize>`: precompiled config for a specific HexaURL byte size.
- `Config::<N>::builder()` to create compiled configs directly.
- Composition and delimiter rule types.

## Usage

```rust
use hexaurl_config::{Composition, Config, DelimiterRules};

let config = Config::<16>::builder()
    .min_length(Some(5))
    .max_length(Some(20))
    .composition(Composition::AlphanumericHyphen)
    .delimiter(Some(
        DelimiterRules::builder()
            .allow_consecutive_hyphens(false)
            .allow_leading_hyphens(false)
            .allow_trailing_hyphens(false)
            .build(),
    ))
    .build()
    .unwrap();

assert_eq!(config.effective_max(), 20);
```

## Defaults

`Config::<N>::default()`:

- Minimum length: `Some(3)`
- Maximum length: capacity-derived max (`N * 4 / 3`)
- Composition: `AlphanumericHyphen`
- Delimiter rules: default (`false` for all delimiter allowances)

`Config::<N>::minimal()`:

- Minimum length: `None`
- Maximum length: capacity-derived max
- Composition: `AlphanumericHyphenUnderscore`
- Delimiter rules: `DelimiterRules::all_allowed()`

## Config Fields

Configurable fields in `Config::<N>::builder()`:

- `min_length(Option<usize>)`
  - Minimum allowed string length
  - `None` disables the lower-bound length check
  - `default`: `Some(3)`

- `max_length(Option<usize>)`
  - Maximum allowed string length
  - Effective max is clamped to `min(max_length, N * 4 / 3)`
  - `None` means `N * 4 / 3` is used as the max
  - `default`: `None`

- `composition(Composition)`
  - Allowed character set
  - `default`: `Composition::AlphanumericHyphen`

- `delimiter(Option<DelimiterRules>)`
  - Delimiter behavior rules
  - `None` uses `DelimiterRules::default()` (all flags are `false`)
  - `default`: `None`

### Build Errors

`build()` returns `Err(ConfigError)` in these cases:

- `InvalidLengthRange { min, max }`
  - `min_length > max_length`

- `InvalidCompiledLengthRange { min, max }`
  - `min_length > effective_max`, where `effective_max` is computed after applying both capacity (`N * 4 / 3`) and `max_length`

## DelimiterRules Fields

Configurable fields in `DelimiterRules::builder()`:

- `allow_leading_hyphens(bool)`
  - Allow `-` at the beginning
  - `default`: `false`

- `allow_trailing_hyphens(bool)`
  - Allow `-` at the end
  - `default`: `false`

- `allow_leading_underscores(bool)`
  - Allow `_` at the beginning
  - `default`: `false`

- `allow_trailing_underscores(bool)`
  - Allow `_` at the end
  - `default`: `false`

- `allow_consecutive_hyphens(bool)`
  - Allow repeated `--`
  - `default`: `false`

- `allow_consecutive_underscores(bool)`
  - Allow repeated `__`
  - `default`: `false`

- `allow_adjacent_hyphen_underscore(bool)`
  - Allow mixed adjacency `-_` or `_-`
  - `default`: `false`

See [the root README.md](https://github.com/perforate-org/hexaurl#readme) for complete documentation.
