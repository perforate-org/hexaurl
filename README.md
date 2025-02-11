# HexaURL

HexaURL is an Rust library that implements a lightweight text format using six-bit character encoding. Designed for case-insensitive URLs and identifiers, it ensures efficient encoding and decoding coupled with robust validation. Optimized for performance-critical environments, HexaURL excels as a high-performance utility for map keys.

## Features

- **Compact Encoding**: Efficiently encodes strings using six-bit characters
- **Extreme Performance**: Engineered for blazing-fast speed with minimal overhead during encoding and decoding, and optimized for efficient use as map keys
- **Case-Insensitive**: Handles both uppercase and lowercase input
- **Configurable Validation**: Flexible validation rules for different use cases
- **Zero-Copy Parsing**: Optimized for performance with minimal allocations
- **Safe and Unsafe APIs**: Both checked and unchecked operations for different performance needs

## Installation

```sh
cargo add hexaurl
```

## Quick Start

```rust
use hexaurl::{encode, decode};

fn main() -> Result<(), hexaurl::Error> {
    let input = "HELLO-WORLD";

    // Encode the string to HexaURL format
    let encoded = encode(input, None)?;

    // Decode back to the original string
    let decoded = decode(&encoded, None)?;

    assert_eq!(decoded, input);
    Ok(())
}
```

## Character Constraints

HexaURL supports the following characters:

- Uppercase letters (A-Z)
- Numbers (0-9)
- Hyphen (-)
- Underscore (\_)

All input is automatically converted to uppercase during encoding.

## Validation Rules

The validation system is highly configurable and supports:

- **String Length**: Configurable minimum and maximum lengths
- **Character Sets**:
  - Alphanumeric only
  - Alphanumeric with hyphens
  - Alphanumeric with underscores
  - Alphanumeric with both hyphens and underscores
- **Delimiter Rules**:
  - Leading/trailing delimiters
  - Consecutive delimiters
  - Adjacent different delimiters

## API Reference

### Encoding Functions

#### `encode(input: &str, config: Option<ValidationConfig>) -> Result<[u8; 16], Error>`

Encodes a string into HexaURL format with validation.

```rust
use hexaurl::encode;

let encoded = encode("HELLO", None)?;
```

#### `encode_unchecked(input: &str) -> [u8; 16]`

Encodes a string without validation for better performance. Use only when input is guaranteed to be valid.

### Decoding Functions

#### `decode(bytes: &[u8; 16], config: Option<Config>) -> Result<String, Error>`

Decodes HexaURL-encoded bytes back into a string with validation.

```rust
use hexaurl::decode;

let decoded = decode(&encoded_bytes, None)?;
```

#### `decode_unchecked(bytes: &[u8; 16]) -> String`

Decodes bytes without validation for better performance. Use only when input is guaranteed to be valid.

### Configuration

The library provides flexible configuration options through `ValidationConfig`:

```rust
use hexaurl_config::validate::{ValidationConfig, IdentifierComposition};

let config = ValidationConfig::builder()
    .min_length(Some(5))
    .max_length(Some(20))
    .identifier(IdentifierComposition::AlphanumericHyphen)
    .build();

let encoded = encode("HELLO-WORLD", Some(config))?;
```

## Performance

HexaURL is designed for high performance:

- Zero-copy parsing where possible
- Minimal allocations
- Optimized bit manipulation operations
- Unsafe options for maximum performance when safety checks aren't needed

## Error Handling

The library provides detailed error types for validation failures:

- `InvalidCharacter`: Character outside allowed set
- `StringTooShort`: Input shorter than minimum length
- `StringTooLong`: Input longer than maximum length
- `ConsecutiveHyphens`: Multiple hyphens in sequence
- `ConsecutiveUnderscores`: Multiple underscores in sequence
- `LeadingTrailingHyphen`: Hyphen at start or end
- `LeadingTrailingUnderscore`: Underscore at start or end
- `AdjacentHyphenUnderscore`: Hyphen next to underscore

## License

This project is licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT License](./LICENSE-MIT) at your option.
