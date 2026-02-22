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

## Performance

- SWAR chunk validation for ASCII checks
- Early return on failure
- Reusable `Config::<N>` for repeated calls

See [the root README.md](https://github.com/perforate-org/hexaurl#readme) for complete documentation.
