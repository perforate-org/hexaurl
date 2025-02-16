// To run the benchmarks:
//
// ```sh
// # Switch to nightly Rust
// rustup override set nightly
//
// # Run benchmarks
// cargo bench --features nightly
// ```
//
// You can keep using stable Rust for normal development since all benchmark code is behind the nightly feature flag.

#![allow(dead_code)]
#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(feature = "nightly")]
mod benches {
    extern crate test;

    use test::{Bencher, black_box};

    use hexaurl::{
        HexaUrl,
        decode::{decode, decode_quick_checked, decode_unchecked},
        encode::{encode, encode_quick_checked, encode_unchecked},
    };
    use hexaurl_validate::validate;
    use once_cell::sync::Lazy;
    use std::collections::{BTreeMap, HashMap};

    const SHORT_INPUT: &str = "hero";
    const MEDIUM_INPUT: &str = "fancy-champ";
    const LONG_INPUT: &str = "ultimate-august-champ";

    static MAP_KEYS: Lazy<Vec<&str>> = Lazy::new(|| {
        include_str!("list.txt")
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect()
    });

    fn prepare_hexaurl_keys() -> Vec<HexaUrl> {
        MAP_KEYS
            .iter()
            .map(|k| unsafe { HexaUrl::new_unchecked(k) })
            .collect()
    }

    fn prepare_string_keys() -> Vec<String> {
        MAP_KEYS.iter().map(|k| k.to_string()).collect()
    }

    // HashMap benchmarks
    #[bench]
    fn hashmap_insert_string(b: &mut Bencher) {
        let keys = prepare_string_keys();
        b.iter(|| {
            let mut map = HashMap::new();
            for (i, key) in keys.iter().enumerate() {
                map.insert(black_box(key).to_ascii_lowercase(), black_box(i));
            }
            black_box(map)
        });
    }

    #[bench]
    fn hashmap_insert_string_with_validate(b: &mut Bencher) {
        let keys = prepare_string_keys();
        b.iter(|| {
            let mut map = HashMap::new();
            for (i, key) in keys.iter().enumerate() {
                if validate::<16>(black_box(key)).is_ok() {
                    map.insert(black_box(key).to_ascii_lowercase(), black_box(i));
                } else {
                    panic!("Invalid key: {}", key);
                }
            }
            black_box(map)
        });
    }

    #[bench]
    fn hashmap_insert_hexaurl(b: &mut Bencher) {
        let keys = prepare_hexaurl_keys();
        b.iter(|| {
            let mut map = HashMap::new();
            for (i, key) in keys.iter().enumerate() {
                map.insert(black_box(key), black_box(i));
            }
            black_box(map)
        });
    }

    #[bench]
    fn hashmap_insert_hexaurl_with_encode(b: &mut Bencher) {
        let keys = prepare_string_keys();
        b.iter(|| {
            let mut map = HashMap::new();
            for (i, key) in keys.iter().enumerate() {
                let encoded = HexaUrl::new(black_box(key)).unwrap();
                map.insert(black_box(encoded), black_box(i));
            }
            black_box(map)
        });
    }

    #[bench]
    fn hashmap_get_string(b: &mut Bencher) {
        let keys = prepare_string_keys();
        let mut map = HashMap::new();
        for (i, key) in keys.iter().enumerate() {
            map.insert(key.to_ascii_lowercase(), i);
        }
        b.iter(|| {
            for key in keys.iter() {
                black_box(map.get(&black_box(key).to_ascii_lowercase())).unwrap();
            }
        });
    }

    #[bench]
    fn hashmap_get_hexaurl(b: &mut Bencher) {
        let keys = prepare_hexaurl_keys();
        let mut map = HashMap::new();
        for (i, key) in keys.iter().enumerate() {
            map.insert(key, i);
        }
        b.iter(|| {
            for key in keys.iter() {
                black_box(map.get(black_box(key))).unwrap();
            }
        });
    }

    #[bench]
    fn hashmap_get_hexaurl_with_encode_quick_checked(b: &mut Bencher) {
        let keys = prepare_hexaurl_keys();
        let mut map = HashMap::new();
        for (i, key) in keys.iter().enumerate() {
            map.insert(key, i);
        }
        let keys = prepare_string_keys();
        b.iter(|| {
            for key in keys.iter() {
                let encoded = HexaUrl::new_quick_checked(black_box(key)).unwrap();
                black_box(map.get(black_box(&encoded))).unwrap();
            }
        });
    }

    // BTreeMap benchmarks
    #[bench]
    fn btreemap_insert_string(b: &mut Bencher) {
        let keys = prepare_string_keys();
        b.iter(|| {
            let mut map = BTreeMap::new();
            for (i, key) in keys.iter().enumerate() {
                map.insert(black_box(key).to_ascii_lowercase(), black_box(i));
            }
            black_box(map)
        });
    }

    #[bench]
    fn btreemap_insert_string_with_validate(b: &mut Bencher) {
        let keys = prepare_string_keys();
        b.iter(|| {
            let mut map = BTreeMap::new();
            for (i, key) in keys.iter().enumerate() {
                if validate::<16>(black_box(key)).is_ok() {
                    map.insert(black_box(key).to_ascii_lowercase(), black_box(i));
                } else {
                    panic!("Invalid key: {}", key);
                }
            }
            black_box(map)
        });
    }

    #[bench]
    fn btreemap_insert_hexaurl(b: &mut Bencher) {
        let keys = prepare_hexaurl_keys();
        b.iter(|| {
            let mut map = BTreeMap::new();
            for (i, key) in keys.iter().enumerate() {
                map.insert(black_box(key), black_box(i));
            }
            black_box(map)
        });
    }

    #[bench]
    fn btreemap_insert_hexaurl_with_encode(b: &mut Bencher) {
        let keys = prepare_string_keys();
        b.iter(|| {
            let mut map = BTreeMap::new();
            for (i, key) in keys.iter().enumerate() {
                let encoded = HexaUrl::new(black_box(key)).unwrap();
                map.insert(black_box(encoded), black_box(i));
            }
            black_box(map)
        });
    }

    #[bench]
    fn btreemap_get_string(b: &mut Bencher) {
        let keys = prepare_string_keys();
        let mut map = BTreeMap::new();
        for (i, key) in keys.iter().enumerate() {
            map.insert(key.to_ascii_lowercase(), i);
        }
        b.iter(|| {
            for key in keys.iter() {
                black_box(map.get(&black_box(key).to_ascii_lowercase())).unwrap();
            }
        });
    }

    #[bench]
    fn btreemap_get_hexaurl(b: &mut Bencher) {
        let keys = prepare_hexaurl_keys();
        let mut map = BTreeMap::new();
        for (i, key) in keys.iter().enumerate() {
            map.insert(key, i);
        }
        b.iter(|| {
            for key in keys.iter() {
                black_box(map.get(black_box(key))).unwrap();
            }
        });
    }

    #[bench]
    fn btreemap_get_hexaurl_with_encode_quick_checked(b: &mut Bencher) {
        let keys = prepare_hexaurl_keys();
        let mut map = BTreeMap::new();
        for (i, key) in keys.iter().enumerate() {
            map.insert(key, i);
        }
        let keys = prepare_string_keys();
        b.iter(|| {
            for key in keys.iter() {
                let encoded = HexaUrl::new_quick_checked(black_box(key)).unwrap();
                black_box(map.get(black_box(&encoded))).unwrap();
            }
        });
    }

    // Validation benchmarks
    #[bench]
    fn validate_short(b: &mut Bencher) {
        b.iter(|| black_box(validate::<16>(black_box(SHORT_INPUT))));
    }

    #[bench]
    fn validate_medium(b: &mut Bencher) {
        b.iter(|| black_box(validate::<16>(black_box(MEDIUM_INPUT))));
    }

    #[bench]
    fn validate_long(b: &mut Bencher) {
        b.iter(|| black_box(validate::<16>(black_box(LONG_INPUT))));
    }

    // Encoding benchmarks
    #[bench]
    fn encode_short(b: &mut Bencher) {
        b.iter(|| black_box(encode::<16>(black_box(SHORT_INPUT))));
    }

    #[bench]
    fn encode_medium(b: &mut Bencher) {
        b.iter(|| black_box(encode::<16>(black_box(MEDIUM_INPUT))));
    }

    #[bench]
    fn encode_long(b: &mut Bencher) {
        b.iter(|| black_box(encode::<16>(black_box(LONG_INPUT))));
    }

    #[bench]
    fn encode_unchecked_short(b: &mut Bencher) {
        b.iter(|| black_box(unsafe { encode_unchecked::<16>(black_box(SHORT_INPUT)) }));
    }

    #[bench]
    fn encode_unchecked_medium(b: &mut Bencher) {
        b.iter(|| black_box(unsafe { encode_unchecked::<16>(black_box(MEDIUM_INPUT)) }));
    }

    #[bench]
    fn encode_unchecked_long(b: &mut Bencher) {
        b.iter(|| black_box(unsafe { encode_unchecked::<16>(black_box(LONG_INPUT)) }));
    }

    // Decoding benchmarks
    #[bench]
    fn decode_short(b: &mut Bencher) {
        let encoded = encode::<16>(SHORT_INPUT).unwrap();
        b.iter(|| black_box(decode::<16, 21>(&encoded)));
    }

    #[bench]
    fn decode_medium(b: &mut Bencher) {
        let encoded = encode::<16>(MEDIUM_INPUT).unwrap();
        b.iter(|| black_box(decode::<16, 21>(&encoded)));
    }

    #[bench]
    fn decode_long(b: &mut Bencher) {
        let encoded = encode::<16>(LONG_INPUT).unwrap();
        b.iter(|| black_box(decode::<16, 21>(&encoded)));
    }

    #[bench]
    fn decode_unchecked_short(b: &mut Bencher) {
        let encoded = encode::<16>(SHORT_INPUT).unwrap();
        b.iter(|| black_box(decode_unchecked::<16, 21>(&encoded)));
    }

    #[bench]
    fn decode_unchecked_medium(b: &mut Bencher) {
        let encoded = encode::<16>(MEDIUM_INPUT).unwrap();
        b.iter(|| black_box(decode_unchecked::<16, 21>(&encoded)));
    }

    #[bench]
    fn decode_unchecked_long(b: &mut Bencher) {
        let encoded = encode::<16>(LONG_INPUT).unwrap();
        b.iter(|| black_box(decode_unchecked::<16, 21>(&encoded)));
    }

    // Encoding safety benchmarks
    #[bench]
    fn encode_quick_checked_short(b: &mut Bencher) {
        b.iter(|| black_box(encode_quick_checked::<16>(black_box(SHORT_INPUT))));
    }

    #[bench]
    fn encode_quick_checked_medium(b: &mut Bencher) {
        b.iter(|| black_box(encode_quick_checked::<16>(black_box(MEDIUM_INPUT))));
    }

    #[bench]
    fn encode_quick_checked_long(b: &mut Bencher) {
        b.iter(|| black_box(encode_quick_checked::<16>(black_box(LONG_INPUT))));
    }

    #[bench]
    fn decode_quick_checked_short(b: &mut Bencher) {
        let encoded = encode::<16>(SHORT_INPUT).unwrap();
        b.iter(|| black_box(decode_quick_checked::<16, 21>(&encoded)));
    }

    #[bench]
    fn decode_quick_checked_medium(b: &mut Bencher) {
        let encoded = encode::<16>(MEDIUM_INPUT).unwrap();
        b.iter(|| black_box(decode_quick_checked::<16, 21>(&encoded)));
    }

    #[bench]
    fn decode_quick_checked_long(b: &mut Bencher) {
        let encoded = encode::<16>(LONG_INPUT).unwrap();
        b.iter(|| black_box(decode_quick_checked::<16, 21>(&encoded)));
    }
}
