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

    use test::Bencher;

    use hexaurl::{
        encode::{encode, encode_unchecked},
        decode::{decode, decode_unchecked},
        HexaUrl
    };
    use hexaurl_validate::{validate, is_encoding_safe};
    use once_cell::sync::Lazy;
    use std::collections::{HashMap, BTreeMap};

    const SHORT_INPUT: &str = "USER";
    const MEDIUM_INPUT: &str = "LOVELY-USER";
    const LONG_INPUT: &str = "GREATEST-LIBERAL-USER";

    static MAP_KEYS: Lazy<Vec<&str>> = Lazy::new(|| {
        include_str!("./names.txt")
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect()
    });

    static FIRST_KEY: Lazy<&str> = Lazy::new(|| MAP_KEYS[0]);

    fn prepare_hexaurl_keys() -> Vec<HexaUrl> {
        MAP_KEYS.iter()
            .map(|k| HexaUrl::new_unchecked(k))
            .collect()
    }

    fn first_key() -> &'static str {
        FIRST_KEY.as_ref()
    }

    fn first_hexaurl_key() -> HexaUrl {
        HexaUrl::new_unchecked(first_key())
    }

    fn prepare_string_keys() -> Vec<String> {
        MAP_KEYS.iter()
            .map(|k| k.to_string())
            .collect()
    }

    // HashMap benchmarks
    #[bench]
    fn bench_hashmap_insert_string(b: &mut Bencher) {
        let keys = prepare_string_keys();
        b.iter(|| {
            let mut map = HashMap::new();
            for (i, key) in keys.iter().cloned().enumerate() {
                map.insert(key, i);
            }
            map
        });
    }

    #[bench]
    fn bench_hashmap_insert_hexaurl(b: &mut Bencher) {
        let keys = prepare_hexaurl_keys();
        b.iter(|| {
            let mut map = HashMap::new();
            for (i, key) in keys.iter().cloned().enumerate() {
                map.insert(key, i);
            }
            map
        });
    }

    #[bench]
    fn bench_hashmap_get_string(b: &mut Bencher) {
        let keys = prepare_string_keys();
        let mut map = HashMap::new();
        for (i, key) in keys.iter().cloned().enumerate() {
            map.insert(key, i);
        }
        let keys = prepare_string_keys();
        b.iter(|| {
            for key in keys.iter() {
                let _ = map.get(key);
            }
        });
    }

    #[bench]
    fn bench_hashmap_get_hexaurl(b: &mut Bencher) {
        let keys = prepare_hexaurl_keys();
        let mut map = HashMap::new();
        for (i, key) in keys.iter().cloned().enumerate() {
            map.insert(key, i);
        }
        let keys = prepare_hexaurl_keys();
        b.iter(|| {
            for key in keys.iter() {
                let _ = map.get(key);
            }
        });
    }

    #[bench]
    fn bench_hashmap_get_one_string(b: &mut Bencher) {
        let keys = prepare_string_keys();
        let mut map = HashMap::new();
        for (i, key) in keys.iter().cloned().enumerate() {
            map.insert(key, i);
        }
        let key = first_key();
        b.iter(|| {
            let _ = map.get(key);
        });
    }

    #[bench]
    fn bench_hashmap_get_one_hexaurl(b: &mut Bencher) {
        let keys = prepare_hexaurl_keys();
        let mut map = HashMap::new();
        for (i, key) in keys.iter().cloned().enumerate() {
            map.insert(key, i);
        }
        let key = first_hexaurl_key();
        b.iter(|| {
            let _ = map.get(&key);
        });
    }

    #[bench]
    fn bench_hashmap_insert_one_string(b: &mut Bencher) {
        let keys = prepare_string_keys();
        let mut map = HashMap::new();
        for (i, key) in keys.iter().cloned().enumerate() {
            map.insert(key, i);
        }
        let key = MEDIUM_INPUT;
        b.iter(|| {
            map.insert(key.to_string(), 0);
        });
    }

    #[bench]
    fn bench_hashmap_insert_one_hexaurl(b: &mut Bencher) {
        let keys = prepare_hexaurl_keys();
        let mut map = HashMap::new();
        for (i, key) in keys.iter().cloned().enumerate() {
            map.insert(key, i);
        }
        let key = HexaUrl::new_unchecked(MEDIUM_INPUT);
        b.iter(|| {
            map.insert(key, 0);
        });
    }

    // BTreeMap benchmarks
    #[bench]
    fn bench_btreemap_insert_string(b: &mut Bencher) {
        let keys = prepare_string_keys();
        b.iter(|| {
            let mut map = BTreeMap::new();
            for (i, key) in keys.iter().cloned().enumerate() {
                map.insert(key, i);
            }
            map
        });
    }

    #[bench]
    fn bench_btreemap_insert_hexaurl(b: &mut Bencher) {
        let keys = prepare_hexaurl_keys();
        b.iter(|| {
            let mut map = BTreeMap::new();
            for (i, key) in keys.iter().cloned().enumerate() {
                map.insert(key, i);
            }
            map
        });
    }

    #[bench]
    fn bench_btreemap_get_string(b: &mut Bencher) {
        let keys = prepare_string_keys();
        let mut map = BTreeMap::new();
        for (i, key) in keys.iter().cloned().enumerate() {
            map.insert(key, i);
        }
        let keys = prepare_string_keys();
        b.iter(|| {
            for key in keys.iter() {
                let _ = map.get(key);
            }
        });
    }

    #[bench]
    fn bench_btreemap_get_hexaurl(b: &mut Bencher) {
        let keys = prepare_hexaurl_keys();
        let mut map = BTreeMap::new();
        for (i, key) in keys.iter().cloned().enumerate() {
            map.insert(key, i);
        }
        let keys = prepare_hexaurl_keys();
        b.iter(|| {
            for key in keys.iter() {
                let _ = map.get(key);
            }
        });
    }

    #[bench]
    fn bench_btreemap_get_one_string(b: &mut Bencher) {
        let keys = prepare_string_keys();
        let mut map = BTreeMap::new();
        for (i, key) in keys.iter().cloned().enumerate() {
            map.insert(key, i);
        }
        let key = first_key();
        b.iter(|| {
            let _ = map.get(key);
        });
    }

    #[bench]
    fn bench_btreemap_get_one_hexaurl(b: &mut Bencher) {
        let keys = prepare_hexaurl_keys();
        let mut map = BTreeMap::new();
        for (i, key) in keys.iter().cloned().enumerate() {
            map.insert(key, i);
        }
        let key = first_hexaurl_key();
        b.iter(|| {
            let _ = map.get(&key);
        });
    }

    #[bench]
    fn bench_btreemap_insert_one_string(b: &mut Bencher) {
        let keys = prepare_string_keys();
        let mut map = BTreeMap::new();
        for (i, key) in keys.iter().cloned().enumerate() {
            map.insert(key, i);
        }
        let key = MEDIUM_INPUT;
        b.iter(|| {
            map.insert(key.to_string(), 0);
        });
    }

    #[bench]
    fn bench_btreemap_insert_one_hexaurl(b: &mut Bencher) {
        let keys = prepare_hexaurl_keys();
        let mut map = BTreeMap::new();
        for (i, key) in keys.iter().cloned().enumerate() {
            map.insert(key, i);
        }
        let key = HexaUrl::new_unchecked(MEDIUM_INPUT);
        b.iter(|| {
            let mut map = BTreeMap::new();
            map.insert(key, 0);
            map
        });
    }

    // Validation benchmarks
    #[bench]
    fn bench_validate_short(b: &mut Bencher) {
        b.iter(|| validate(SHORT_INPUT, None));
    }

    #[bench]
    fn bench_validate_medium(b: &mut Bencher) {
        b.iter(|| validate(MEDIUM_INPUT, None));
    }

    #[bench]
    fn bench_validate_long(b: &mut Bencher) {
        b.iter(|| validate(LONG_INPUT, None));
    }

    // Encoding benchmarks
    #[bench]
    fn bench_encode_short(b: &mut Bencher) {
        b.iter(|| encode(SHORT_INPUT, None));
    }

    #[bench]
    fn bench_encode_medium(b: &mut Bencher) {
        b.iter(|| encode(MEDIUM_INPUT, None));
    }

    #[bench]
    fn bench_encode_long(b: &mut Bencher) {
        b.iter(|| encode(LONG_INPUT, None));
    }

    #[bench]
    fn bench_encode_unchecked_short(b: &mut Bencher) {
        b.iter(|| encode_unchecked(SHORT_INPUT));
    }

    #[bench]
    fn bench_encode_unchecked_medium(b: &mut Bencher) {
        b.iter(|| encode_unchecked(MEDIUM_INPUT));
    }

    #[bench]
    fn bench_encode_unchecked_long(b: &mut Bencher) {
        b.iter(|| encode_unchecked(LONG_INPUT));
    }

    // Decoding benchmarks
    #[bench]
    fn bench_decode_short(b: &mut Bencher) {
        let encoded = encode(SHORT_INPUT, None).unwrap();
        b.iter(|| decode(&encoded, None));
    }

    #[bench]
    fn bench_decode_medium(b: &mut Bencher) {
        let encoded = encode(MEDIUM_INPUT, None).unwrap();
        b.iter(|| decode(&encoded, None));
    }

    #[bench]
    fn bench_decode_long(b: &mut Bencher) {
        let encoded = encode(LONG_INPUT, None).unwrap();
        b.iter(|| decode(&encoded, None));
    }

    #[bench]
    fn bench_decode_unchecked_short(b: &mut Bencher) {
        let encoded = encode(SHORT_INPUT, None).unwrap();
        b.iter(|| decode_unchecked(&encoded));
    }

    #[bench]
    fn bench_decode_unchecked_medium(b: &mut Bencher) {
        let encoded = encode(MEDIUM_INPUT, None).unwrap();
        b.iter(|| decode_unchecked(&encoded));
    }

    #[bench]
    fn bench_decode_unchecked_long(b: &mut Bencher) {
        let encoded = encode(LONG_INPUT, None).unwrap();
        b.iter(|| decode_unchecked(&encoded));
    }

    #[bench]
    fn bench_new_string_short(b: &mut Bencher) {
        b.iter(|| SHORT_INPUT.to_owned());
    }

    #[bench]
    fn bench_new_string_medium(b: &mut Bencher) {
        b.iter(|| MEDIUM_INPUT.to_owned());
    }

    #[bench]
    fn bench_new_string_long(b: &mut Bencher) {
        b.iter(|| LONG_INPUT.to_owned());
    }

    // Encoding safety benchmarks
    #[bench]
    fn bench_is_encoding_safe_and_encode_unchecked_short(b: &mut Bencher) {
        b.iter(|| {
            if is_encoding_safe(SHORT_INPUT) {
                encode_unchecked(SHORT_INPUT);
            }
        });
    }

    #[bench]
    fn bench_is_encoding_safe_and_encode_unchecked_medium(b: &mut Bencher) {
        b.iter(|| {
            if is_encoding_safe(MEDIUM_INPUT) {
                encode_unchecked(MEDIUM_INPUT);
            }
        });
    }

    #[bench]
    fn bench_is_encoding_safe_and_encode_unchecked_long(b: &mut Bencher) {
        b.iter(|| {
            if is_encoding_safe(LONG_INPUT) {
                encode_unchecked(LONG_INPUT);
            }
        });
    }
}
