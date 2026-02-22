# hexaurl-validate

Validation functionality for the HexaURL format.

## Usage

```rust
use hexaurl_config::{Composition, Config};
use hexaurl_validate::{validate, validate_with_config};

let result = validate::<16>("Hello-World");
assert!(result.is_ok());

let config = Config::<16>::builder()
    .min_length(Some(5))
    .composition(Composition::AlphanumericHyphen)
    .build()
    .unwrap();

let result = validate_with_config::<16>("Hello-World", &config);
assert!(result.is_ok());
```

## Configuration Details

`validate_with_config::<N>(..., &config)` expects `hexaurl_config::Config<N>`.
You can reuse the same compiled config across many validation calls.

### Config Fields

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

### DelimiterRules Fields

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

## Performance

- SWAR chunk validation for ASCII checks
- Early return on failure
- Reusable `Config::<N>` for repeated calls

See [the root README.md](https://github.com/perforate-org/hexaurl#readme) for complete documentation.
