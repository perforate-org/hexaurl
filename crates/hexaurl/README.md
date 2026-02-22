# hexaurl

Core encoding and decoding functionality for the HexaURL format, designed for performance and safety.

## Features

This crate delivers a robust implementation that includes:

- Six-bit character encoding and decoding
- Efficient bit-packing operations
- Safe APIs with comprehensive validation
- Unsafe APIs for unmatched performance in trusted contexts
- An optional struct-based API enabled by default with the `struct-api` feature
- `serde` support is also enabled by default

## Usage

```rust,ignore
use hexaurl::{encode, decode};

// Encode a string with built-in validation
let input = "hello-world";
let encoded: [u8; 16] = encode(input)?;

// Decode back to a string with validation
let decoded = decode::<16, 21>(&encoded)?;
assert_eq!(decoded, input);
```

### Struct Usage

Employing the `HexaUrl` struct as a key in collections like HashMap or BTreeMap not only streamlines lookup operations but also can yield better performance than using a standard String. While the encoding process itself is extremely fast, the extra validation can add nanoseconds overhead. Therefore, the API offers both configurable strict validation and minimal validation (to avoid panics and collisions during encoding).

For instance, if your application encodes Strings received from an API into HexaUrl, you can enforce rigorous checks at insert time and then use minimal validation during look-ups to optimize performance:

```rust
use std::collections::HashMap;
use hexaurl::HexaUrl;

let input = "abc123";
let mut map = HashMap::new();

// Insert a value using the safe constructor with full validation.
let safe_key = HexaUrl::new(input)
    .expect("Input should be valid for safe encoding");
map.insert(safe_key, 42);

// Retrieve the value using the unchecked constructor when the input is known to be safe.
let retrieved = if let Ok(key) = HexaUrl::new_quick(input) {
    map.get(&key)
} else {
    None
};

assert_eq!(retrieved, Some(&42));
```

## Implementation Details

The encoder transforms strings into a compact binary format through the following process:

- Maps each character to a 6-bit value using SIXBIT encoding.
- Packs four characters into three bytes.
- Supports a maximum input length of 21 characters.
- Produces fixed-size 16-byte arrays.

The decoder reverses this process to recover the original string:

- Unpacks three bytes into four characters.
- Correctly handles any partial chunks at the end of the input.
- Returns lowercase normalized UTF-8 strings.

## Safety

The safe APIs (`encode` and `decode`) enforce:

- Strict character validation
- Rigorous length checking
- UTF-8 compliance verification
- Customizable configuration options

Conversely, the unsafe APIs (`encode_unchecked` and `decode_unchecked`):

- Omit all validation checks for maximum speed.
- Are intended only for scenarios with pre-validated input.
- Can lead to undefined behavior if used with invalid data.
- Deliver peak performance when used correctly.

For a complete guide, please refer to [the root README.md](https://github.com/perforate-org/hexaurl#readme).
