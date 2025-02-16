# HexaURL

![HexaURL logo](https://github.com/perforate-org/hexaurl/blob/main/assets/logo.png?raw=true)

HexaURL is a character code that implements a lightweight text format using six-bit character encoding. Designed for case-insensitive URLs and identifiers, it ensures efficient encoding and decoding coupled with robust validation. Optimized for performance-critical environments, HexaURL excels as a high-performance utility for map keys.

## Features

- **Compact Encoding**: Efficiently encodes strings using six-bit characters
- **Extreme Performance**: Engineered for blazing-fast speed with minimal overhead during encoding and decoding, and optimized for efficient use as collection keys
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
use hexaurl::HexaUrl;

fn main() -> Result<(), hexaurl::Error> {
    let input = "Hello-World";

    // Encode the string to HexaURL format
    let encoded = HexaUrl::new(input)?;

    assert_eq!(encoded.to_string(), input.to_lowercase());
    Ok(())
}
```

## Character Constraints

HexaURL supports the following characters:

- Lowercase letters (a-z)
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

## Performance

HexaURL is designed for high performance:

- Zero-copy parsing where possible
- Minimal allocations
- Optimized bit manipulation operations

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
