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
            .allow_leading_trailing_hyphens(false)
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

See [the root README.md](https://github.com/perforate-org/hexaurl#readme) for complete documentation.
