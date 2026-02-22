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

    use test::{black_box, Bencher};

    use fixedstr::str32;
    use hexaurl::{
        decode::{decode, decode_into, decode_unchecked, decode_unchecked_into},
        encode::{encode, encode_quick, encode_unchecked},
        HexaUrl,
    };
    use hexaurl_validate::{
        config::{Composition, Config, DelimiterRules},
        validate, validate_for_lookup, validate_with_compiled_config, validate_with_config,
    };
    use once_cell::sync::Lazy;
    use std::collections::{BTreeMap, HashMap};

    const SHORT_INPUT: &str = "hero";
    const MEDIUM_INPUT: &str = "fancy-champ";
    const LONG_INPUT: &str = "ultimate-august-champ";
    const DELIM_HEAVY_HYPHEN: &str = "a-a-a-a-a-a-a-a";
    const DELIM_HEAVY_UNDERSCORE: &str = "a_a_a_a_a_a_a_a";
    const DELIM_MIXED: &str = "a-_a-_a-_a-_a";
    const ERROR_INVALID_CHAR: &str = "bad.input.value";
    const ERROR_CONSEC_HYPHEN: &str = "bad--input--value";
    const ERROR_CONSEC_UNDERSCORE: &str = "bad__input__value";

    static CFG_ALNUM: Lazy<Config<16>> = Lazy::new(|| {
        Config::<16>::builder()
            .composition(Composition::Alphanumeric)
            .build()
            .unwrap()
    });
    static CFG_ALNUM_HYPHEN: Lazy<Config<16>> = Lazy::new(|| {
        Config::<16>::builder()
            .composition(Composition::AlphanumericHyphen)
            .build()
            .unwrap()
    });
    static CFG_ALNUM_UNDERSCORE: Lazy<Config<16>> = Lazy::new(|| {
        Config::<16>::builder()
            .composition(Composition::AlphanumericUnderscore)
            .build()
            .unwrap()
    });
    static CFG_ALNUM_BOTH: Lazy<Config<16>> = Lazy::new(|| {
        Config::<16>::builder()
            .composition(Composition::AlphanumericHyphenUnderscore)
            .build()
            .unwrap()
    });
    static CFG_STRICT_HYPHEN: Lazy<Config<16>> = Lazy::new(|| {
        Config::<16>::builder()
            .composition(Composition::AlphanumericHyphen)
            .delimiter(Some(DelimiterRules::new(false, false, false, false, false)))
            .build()
            .unwrap()
    });
    static CFG_STRICT_UNDERSCORE: Lazy<Config<16>> = Lazy::new(|| {
        Config::<16>::builder()
            .composition(Composition::AlphanumericUnderscore)
            .delimiter(Some(DelimiterRules::new(false, false, false, false, false)))
            .build()
            .unwrap()
    });
    static COMPILED_CFG_ALNUM_HYPHEN: Lazy<Config<16>> = Lazy::new(|| *CFG_ALNUM_HYPHEN);
    static COMPILED_CFG_ALNUM_BOTH: Lazy<Config<16>> = Lazy::new(|| *CFG_ALNUM_BOTH);
    static COMPILED_CFG_ALNUM: Lazy<Config<16>> = Lazy::new(|| *CFG_ALNUM);
    static COMPILED_CFG_ALNUM_UNDERSCORE: Lazy<Config<16>> = Lazy::new(|| *CFG_ALNUM_UNDERSCORE);
    static COMPILED_CFG_STRICT_HYPHEN: Lazy<Config<16>> = Lazy::new(|| *CFG_STRICT_HYPHEN);
    static COMPILED_CFG_STRICT_UNDERSCORE: Lazy<Config<16>> = Lazy::new(|| *CFG_STRICT_UNDERSCORE);

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

    fn prepare_fixedstr_keys() -> Vec<str32> {
        MAP_KEYS.iter().map(|k| str32::make(k)).collect()
    }

    #[bench]
    fn len_hexaurl(b: &mut Bencher) {
        let value = unsafe { HexaUrl::new_unchecked(MEDIUM_INPUT) }
            .as_bytes()
            .to_owned();
        b.iter(|| {
            let value = unsafe { HexaUrl::from_slice(black_box(&value)) };
            black_box(value.len())
        });
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
    fn hashmap_insert_fixedstr(b: &mut Bencher) {
        let keys = prepare_fixedstr_keys();
        b.iter(|| {
            let mut map = HashMap::new();
            for (i, key) in keys.iter().enumerate() {
                if key.is_ascii() {
                    map.insert(black_box(key).to_ascii_lower(), black_box(i));
                } else {
                    panic!("Invalid key: {}", key);
                }
            }
            black_box(map)
        });
    }

    #[bench]
    fn hashmap_insert_fixedstr_with_make_and_validate(b: &mut Bencher) {
        let keys = prepare_string_keys();
        b.iter(|| {
            let mut map = HashMap::new();
            for (i, key) in keys.iter().enumerate() {
                if validate::<16>(black_box(key)).is_ok() {
                    map.insert(str32::make(black_box(key)).to_ascii_lower(), black_box(i));
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
    fn hashmap_get_fixedstr(b: &mut Bencher) {
        let keys = prepare_fixedstr_keys();
        let mut map = HashMap::new();
        for (i, key) in keys.iter().enumerate() {
            if key.is_ascii() {
                map.insert(key.to_ascii_lower(), i);
            } else {
                panic!("Invalid key: {}", key);
            }
        }
        b.iter(|| {
            for key in keys.iter() {
                if key.is_ascii() {
                    black_box(map.get(&black_box(key).to_ascii_lower())).unwrap();
                } else {
                    panic!("Invalid key: {}", key);
                }
            }
        });
    }

    #[bench]
    fn hashmap_get_fixedstr_with_make(b: &mut Bencher) {
        let keys = prepare_fixedstr_keys();
        let mut map = HashMap::new();
        for (i, key) in keys.iter().enumerate() {
            if key.is_ascii() {
                map.insert(key.to_ascii_lower(), i);
            } else {
                panic!("Invalid key: {}", key);
            }
        }
        let keys = prepare_string_keys();
        b.iter(|| {
            for key in keys.iter() {
                let key = str32::make(black_box(key));
                if key.is_ascii() {
                    black_box(map.get(&key.to_ascii_lower())).unwrap();
                } else {
                    panic!("Invalid key: {}", key);
                }
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
    fn hashmap_get_hexaurl_with_encode_quick(b: &mut Bencher) {
        let keys = prepare_hexaurl_keys();
        let mut map = HashMap::new();
        for (i, key) in keys.iter().enumerate() {
            map.insert(key, i);
        }
        let keys = prepare_string_keys();
        b.iter(|| {
            for key in keys.iter() {
                let encoded = HexaUrl::new_quick(black_box(key)).unwrap();
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
    fn btreemap_insert_fixedstr(b: &mut Bencher) {
        let keys = prepare_fixedstr_keys();
        b.iter(|| {
            let mut map = BTreeMap::new();
            for (i, key) in keys.iter().enumerate() {
                map.insert(black_box(key).to_ascii_lower(), black_box(i));
            }
            black_box(map)
        });
    }

    #[bench]
    fn btreemap_insert_fixedstr_with_make_and_validate(b: &mut Bencher) {
        let keys = prepare_string_keys();
        b.iter(|| {
            let mut map = BTreeMap::new();
            for (i, key) in keys.iter().enumerate() {
                if validate::<16>(black_box(key)).is_ok() {
                    map.insert(str32::make(black_box(key)).to_ascii_lower(), black_box(i));
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
    fn btreemap_get_fixedstr(b: &mut Bencher) {
        let keys = prepare_fixedstr_keys();
        let mut map = BTreeMap::new();
        for (i, key) in keys.iter().enumerate() {
            if key.is_ascii() {
                map.insert(key.to_ascii_lower(), i);
            } else {
                panic!("Invalid key: {}", key);
            }
        }
        b.iter(|| {
            for key in keys.iter() {
                if key.is_ascii() {
                    black_box(map.get(&black_box(key).to_ascii_lower())).unwrap();
                } else {
                    panic!("Invalid key: {}", key);
                }
            }
        });
    }

    #[bench]
    fn btreemap_get_fixedstr_with_make(b: &mut Bencher) {
        let keys = prepare_fixedstr_keys();
        let mut map = BTreeMap::new();
        for (i, key) in keys.iter().enumerate() {
            if key.is_ascii() {
                map.insert(key.to_ascii_lower(), i);
            } else {
                panic!("Invalid key: {}", key);
            }
        }
        let keys = prepare_string_keys();
        b.iter(|| {
            for key in keys.iter() {
                let key = str32::make(black_box(key));
                if key.is_ascii() {
                    black_box(map.get(&key.to_ascii_lower())).unwrap();
                } else {
                    panic!("Invalid key: {}", key);
                }
            }
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
    fn btreemap_get_hexaurl_with_encode_quick(b: &mut Bencher) {
        let keys = prepare_hexaurl_keys();
        let mut map = BTreeMap::new();
        for (i, key) in keys.iter().enumerate() {
            map.insert(key, i);
        }
        let keys = prepare_string_keys();
        b.iter(|| {
            for key in keys.iter() {
                let encoded = HexaUrl::new_quick(black_box(key)).unwrap();
                black_box(map.get(&encoded)).unwrap();
            }
        });
    }

    // Validation benchmarks
    #[bench]
    fn validate_short(b: &mut Bencher) {
        b.iter(|| validate::<16>(black_box(SHORT_INPUT)));
    }

    #[bench]
    fn validate_medium(b: &mut Bencher) {
        b.iter(|| validate::<16>(black_box(MEDIUM_INPUT)));
    }

    #[bench]
    fn validate_long(b: &mut Bencher) {
        b.iter(|| validate::<16>(black_box(LONG_INPUT)));
    }

    // Validation benchmarks: delimiter-heavy workload
    #[bench]
    fn validate_delimiter_heavy_hyphen(b: &mut Bencher) {
        b.iter(|| {
            validate_with_config::<16>(
                black_box(DELIM_HEAVY_HYPHEN),
                black_box(&*COMPILED_CFG_ALNUM_HYPHEN),
            )
        });
    }

    #[bench]
    fn validate_delimiter_heavy_underscore(b: &mut Bencher) {
        b.iter(|| {
            validate_with_config::<16>(
                black_box(DELIM_HEAVY_UNDERSCORE),
                black_box(&*COMPILED_CFG_ALNUM_UNDERSCORE),
            )
        });
    }

    #[bench]
    fn validate_delimiter_heavy_mixed(b: &mut Bencher) {
        b.iter(|| {
            validate_with_config::<16>(black_box(DELIM_MIXED), black_box(&*COMPILED_CFG_ALNUM_BOTH))
        });
    }

    // Validation benchmarks: error-heavy workload
    #[bench]
    fn validate_error_invalid_char(b: &mut Bencher) {
        b.iter(|| {
            validate_with_config::<16>(
                black_box(ERROR_INVALID_CHAR),
                black_box(&*COMPILED_CFG_ALNUM_HYPHEN),
            )
        });
    }

    #[bench]
    fn validate_error_consecutive_hyphen(b: &mut Bencher) {
        b.iter(|| {
            validate_with_config::<16>(
                black_box(ERROR_CONSEC_HYPHEN),
                black_box(&*COMPILED_CFG_STRICT_HYPHEN),
            )
        });
    }

    #[bench]
    fn validate_error_consecutive_underscore(b: &mut Bencher) {
        b.iter(|| {
            validate_with_config::<16>(
                black_box(ERROR_CONSEC_UNDERSCORE),
                black_box(&*COMPILED_CFG_STRICT_UNDERSCORE),
            )
        });
    }

    // Validation benchmarks: composition-specific workload
    #[bench]
    fn validate_comp_alnum(b: &mut Bencher) {
        b.iter(|| {
            validate_with_config::<16>(black_box("abc123xyz"), black_box(&*COMPILED_CFG_ALNUM))
        });
    }

    #[bench]
    fn validate_comp_alnum_hyphen(b: &mut Bencher) {
        b.iter(|| {
            validate_with_config::<16>(
                black_box("abc-123-xyz"),
                black_box(&*COMPILED_CFG_ALNUM_HYPHEN),
            )
        });
    }

    #[bench]
    fn validate_comp_alnum_underscore(b: &mut Bencher) {
        b.iter(|| {
            validate_with_config::<16>(
                black_box("abc_123_xyz"),
                black_box(&*COMPILED_CFG_ALNUM_UNDERSCORE),
            )
        });
    }

    #[bench]
    fn validate_comp_alnum_both(b: &mut Bencher) {
        b.iter(|| {
            validate_with_config::<16>(
                black_box("abc-123_xyz"),
                black_box(&*COMPILED_CFG_ALNUM_BOTH),
            )
        });
    }

    #[bench]
    fn validate_compiled_delimiter_heavy_hyphen(b: &mut Bencher) {
        b.iter(|| {
            validate_with_compiled_config::<16>(
                black_box(DELIM_HEAVY_HYPHEN),
                black_box(&*COMPILED_CFG_ALNUM_HYPHEN),
            )
        });
    }

    #[bench]
    fn validate_compiled_delimiter_heavy_mixed(b: &mut Bencher) {
        b.iter(|| {
            validate_with_compiled_config::<16>(
                black_box(DELIM_MIXED),
                black_box(&*COMPILED_CFG_ALNUM_BOTH),
            )
        });
    }

    #[bench]
    fn validate_lookup_safe_short(b: &mut Bencher) {
        b.iter(|| validate_for_lookup::<16>(black_box(SHORT_INPUT)));
    }

    #[bench]
    fn validate_lookup_safe_medium(b: &mut Bencher) {
        b.iter(|| validate_for_lookup::<16>(black_box(MEDIUM_INPUT)));
    }

    // Encoding benchmarks
    #[bench]
    fn encode_short(b: &mut Bencher) {
        b.iter(|| encode::<16>(black_box(SHORT_INPUT)));
    }

    #[bench]
    fn encode_medium(b: &mut Bencher) {
        b.iter(|| encode::<16>(black_box(MEDIUM_INPUT)));
    }

    #[bench]
    fn encode_long(b: &mut Bencher) {
        b.iter(|| encode::<16>(black_box(LONG_INPUT)));
    }

    #[bench]
    fn encode_unchecked_short(b: &mut Bencher) {
        b.iter(|| unsafe { encode_unchecked::<16>(black_box(SHORT_INPUT)) });
    }

    #[bench]
    fn encode_unchecked_medium(b: &mut Bencher) {
        b.iter(|| unsafe { encode_unchecked::<16>(black_box(MEDIUM_INPUT)) });
    }

    #[bench]
    fn encode_unchecked_long(b: &mut Bencher) {
        b.iter(|| unsafe { encode_unchecked::<16>(black_box(LONG_INPUT)) });
    }

    // Decoding benchmarks
    #[bench]
    fn decode_short(b: &mut Bencher) {
        let encoded = encode::<16>(SHORT_INPUT).unwrap();
        b.iter(|| decode::<16, 21>(black_box(&encoded)));
    }

    #[bench]
    fn decode_medium(b: &mut Bencher) {
        let encoded = encode::<16>(MEDIUM_INPUT).unwrap();
        b.iter(|| decode::<16, 21>(black_box(&encoded)));
    }

    #[bench]
    fn decode_long(b: &mut Bencher) {
        let encoded = encode::<16>(LONG_INPUT).unwrap();
        b.iter(|| decode::<16, 21>(black_box(&encoded)));
    }

    #[bench]
    fn decode_unchecked_short(b: &mut Bencher) {
        let encoded = encode::<16>(SHORT_INPUT).unwrap();
        b.iter(|| decode_unchecked::<16, 21>(black_box(&encoded)));
    }

    #[bench]
    fn decode_unchecked_medium(b: &mut Bencher) {
        let encoded = encode::<16>(MEDIUM_INPUT).unwrap();
        b.iter(|| decode_unchecked::<16, 21>(black_box(&encoded)));
    }

    #[bench]
    fn decode_unchecked_long(b: &mut Bencher) {
        let encoded = encode::<16>(LONG_INPUT).unwrap();
        b.iter(|| decode_unchecked::<16, 21>(black_box(&encoded)));
    }

    #[bench]
    fn decode_into_short(b: &mut Bencher) {
        let encoded = encode::<16>(SHORT_INPUT).unwrap();
        b.iter(|| {
            let mut out = [0u8; 21];
            let decoded = decode_into::<16, 21>(black_box(&encoded), black_box(&mut out)).unwrap();
            black_box(decoded.len());
        });
    }

    #[bench]
    fn decode_into_medium(b: &mut Bencher) {
        let encoded = encode::<16>(MEDIUM_INPUT).unwrap();
        b.iter(|| {
            let mut out = [0u8; 21];
            let decoded = decode_into::<16, 21>(black_box(&encoded), black_box(&mut out)).unwrap();
            black_box(decoded.len());
        });
    }

    #[bench]
    fn decode_into_long(b: &mut Bencher) {
        let encoded = encode::<16>(LONG_INPUT).unwrap();
        b.iter(|| {
            let mut out = [0u8; 21];
            let decoded = decode_into::<16, 21>(black_box(&encoded), black_box(&mut out)).unwrap();
            black_box(decoded.len());
        });
    }

    #[bench]
    fn decode_unchecked_into_short(b: &mut Bencher) {
        let encoded = encode::<16>(SHORT_INPUT).unwrap();
        b.iter(|| {
            let mut out = [0u8; 21];
            let decoded = decode_unchecked_into::<16, 21>(black_box(&encoded), black_box(&mut out));
            black_box(decoded.len());
        });
    }

    #[bench]
    fn decode_unchecked_into_medium(b: &mut Bencher) {
        let encoded = encode::<16>(MEDIUM_INPUT).unwrap();
        b.iter(|| {
            let mut out = [0u8; 21];
            let decoded = decode_unchecked_into::<16, 21>(black_box(&encoded), black_box(&mut out));
            black_box(decoded.len());
        });
    }

    #[bench]
    fn decode_unchecked_into_long(b: &mut Bencher) {
        let encoded = encode::<16>(LONG_INPUT).unwrap();
        b.iter(|| {
            let mut out = [0u8; 21];
            let decoded = decode_unchecked_into::<16, 21>(black_box(&encoded), black_box(&mut out));
            black_box(decoded.len());
        });
    }

    // Encoding safety benchmarks
    #[bench]
    fn encode_quick_short(b: &mut Bencher) {
        b.iter(|| encode_quick::<16>(black_box(SHORT_INPUT)));
    }

    #[bench]
    fn encode_quick_medium(b: &mut Bencher) {
        b.iter(|| encode_quick::<16>(black_box(MEDIUM_INPUT)));
    }

    #[bench]
    fn encode_quick_long(b: &mut Bencher) {
        b.iter(|| encode_quick::<16>(black_box(LONG_INPUT)));
    }
}
