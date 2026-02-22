extern crate test;

use super::*;
use canbench_rs::bench;
use hexaurl::config::{Composition, Config, DelimiterRules};
use hexaurl::encode_with_config;
use hexaurl::validate::validate_with_compiled_config;
use once_cell::sync::Lazy;
use test::black_box;

const INPUT: &str = "hello-world";
const ENCODE_HYPHEN_STRICT_OK: &str = "ab-cd-ef-gh";
const ENCODE_MIXED_STRICT_OK: &str = "ab-cd_ef-gh_ij";
const ENCODE_MIXED_PERMISSIVE_OK: &str = "ab-_cd-_ef-_gh";
const ENCODE_ERROR_ADJACENT_MIXED: &str = "ab-_cd-ef-gh";
const ENCODE_ERROR_CONSEC_HYPHEN: &str = "ab--cd-ef-gh";
const VALIDATE_SHORT_OK: &str = "abc";
const VALIDATE_MEDIUM_OK: &str = "hello-world_12";
const VALIDATE_LONG_OK: &str = "abcd-efgh_ijkl-mnop";
const VALIDATE_DELIM_HEAVY_OK: &str = "a-b-c-d-e-f-g-h-i-j-k";
const VALIDATE_MIXED_STRICT_OK: &str = "ab-cd_ef-gh_ij";
const VALIDATE_ERROR_INVALID_CHAR: &str = "hello*world";
const VALIDATE_ERROR_CONSEC_HYPHEN: &str = "ab--cd-ef-gh";
const VALIDATE_ERROR_ADJACENT_MIXED: &str = "ab-_cd-ef-gh";
static INPUT_ENCODED: Lazy<HexaUrl> = Lazy::new(|| HexaUrl::new(black_box(INPUT)).unwrap());
static FIRST_100_KEYS: Lazy<Vec<String>> =
    Lazy::new(|| MAP_KEYS.iter().take(100).map(|k| k.to_string()).collect());
static FIRST_100_HEX: Lazy<Vec<HexaUrl>> = Lazy::new(|| {
    FIRST_100_KEYS
        .iter()
        .map(|k| HexaUrl::new(k).unwrap())
        .collect()
});
static CFG_ALNUM: Lazy<Config<16>> = Lazy::new(|| {
    Config::<16>::builder()
        .min_length(Some(3))
        .composition(Composition::Alphanumeric)
        .build()
        .unwrap()
});
static CFG_HYPHEN_STRICT: Lazy<Config<16>> = Lazy::new(|| {
    Config::<16>::builder()
        .min_length(Some(3))
        .composition(Composition::AlphanumericHyphen)
        .build()
        .unwrap()
});
static CFG_HYPHEN_UNDERSCORE_PERMISSIVE: Lazy<Config<16>> = Lazy::new(|| {
    Config::<16>::builder()
        .min_length(Some(3))
        .composition(Composition::AlphanumericHyphenUnderscore)
        .delimiter(Some(DelimiterRules::all_allowed()))
        .build()
        .unwrap()
});
static CFG_HYPHEN_UNDERSCORE_STRICT: Lazy<Config<16>> = Lazy::new(|| {
    Config::<16>::builder()
        .min_length(Some(3))
        .composition(Composition::AlphanumericHyphenUnderscore)
        .build()
        .unwrap()
});

#[bench]
fn bench_encode() {
    let res = HexaUrl::new(black_box(INPUT)).unwrap();
    black_box(res);
}

#[bench]
fn bench_encode_quick() {
    let res = HexaUrl::new_quick(black_box(INPUT)).unwrap();
    black_box(res);
}

#[bench]
fn bench_encode_unchecked() {
    unsafe {
        let res = HexaUrl::new_unchecked(black_box(INPUT));
        black_box(res);
    }
}

#[bench]
fn bench_encode_with_config_hyphen_strict_ok() {
    let res = encode_with_config::<16>(
        black_box(ENCODE_HYPHEN_STRICT_OK),
        black_box(&*CFG_HYPHEN_STRICT),
    );
    let _ = black_box(res);
}

#[bench]
fn bench_encode_with_config_mixed_strict_ok() {
    let res = encode_with_config::<16>(
        black_box(ENCODE_MIXED_STRICT_OK),
        black_box(&*CFG_HYPHEN_UNDERSCORE_STRICT),
    );
    let _ = black_box(res);
}

#[bench]
fn bench_encode_with_config_mixed_permissive_ok() {
    let res = encode_with_config::<16>(
        black_box(ENCODE_MIXED_PERMISSIVE_OK),
        black_box(&*CFG_HYPHEN_UNDERSCORE_PERMISSIVE),
    );
    let _ = black_box(res);
}

#[bench]
fn bench_encode_with_config_error_adjacent_mixed_strict() {
    let res = encode_with_config::<16>(
        black_box(ENCODE_ERROR_ADJACENT_MIXED),
        black_box(&*CFG_HYPHEN_UNDERSCORE_STRICT),
    );
    let _ = black_box(res);
}

#[bench]
fn bench_encode_with_config_error_consecutive_hyphen_strict() {
    let res = encode_with_config::<16>(
        black_box(ENCODE_ERROR_CONSEC_HYPHEN),
        black_box(&*CFG_HYPHEN_STRICT),
    );
    let _ = black_box(res);
}

#[bench]
fn bench_decode() {
    let res = black_box(INPUT_ENCODED.to_string());
    black_box(res);
}

#[bench]
fn bench_decode_unchecked() {
    let res = black_box(INPUT_ENCODED.decode_unchecked());
    black_box(res);
}

#[bench]
fn bench_get_actor_by_hex() {
    FIRST_100_HEX.iter().for_each(|k| {
        let res = black_box(get_actor_by_hex(black_box(*k)));
        black_box(res);
    });
}

#[bench]
fn bench_get_actor_by_hex_string() {
    FIRST_100_KEYS.iter().cloned().for_each(|k| {
        let res = black_box(get_actor_by_hex_string(black_box(k)));
        black_box(res);
    });
}

#[bench]
fn bench_get_actor_by_plain_string() {
    FIRST_100_KEYS.iter().cloned().for_each(|k| {
        let res = black_box(get_actor_by_plain_string(black_box(k)));
        black_box(res);
    });
}

#[bench]
fn bench_get_actor_stable_by_hex() {
    FIRST_100_HEX.iter().for_each(|k| {
        let res = black_box(get_actor_stable_by_hex(black_box(*k)));
        black_box(res);
    });
}

#[bench]
fn bench_get_actor_stable_by_hex_string() {
    FIRST_100_KEYS.iter().cloned().for_each(|k| {
        let res = black_box(get_actor_stable_by_hex_string(black_box(k)));
        black_box(res);
    });
}

#[bench]
fn bench_get_actor_stable_by_plain_string() {
    FIRST_100_KEYS.iter().cloned().for_each(|k| {
        let res = black_box(get_actor_stable_by_plain_string(black_box(k)));
        black_box(res);
    });
}

#[bench]
fn bench_insert_actor_by_hex() {
    let actor = Principal::anonymous();
    FIRST_100_HEX.iter().for_each(|k| {
        let res = black_box(insert_actor_by_hex((black_box(*k), black_box(actor))));
        black_box(res);
    });
}

#[bench]
fn bench_insert_actor_by_hex_string() {
    let actor = Principal::anonymous();
    FIRST_100_KEYS.iter().cloned().for_each(|k| {
        let res = black_box(insert_actor_by_hex_string((black_box(k), black_box(actor))));
        black_box(res);
    });
}

#[bench]
fn bench_insert_actor_by_plain_string() {
    let actor = Principal::anonymous();
    FIRST_100_KEYS.iter().cloned().for_each(|k| {
        let res = black_box(insert_actor_by_plain_string((
            black_box(k),
            black_box(actor),
        )));
        black_box(res);
    });
}

#[bench]
fn bench_insert_actor_stable_by_hex() {
    let actor = Principal::anonymous();
    FIRST_100_HEX.iter().for_each(|k| {
        let res = black_box(insert_actor_stable_by_hex((
            black_box(*k),
            black_box(actor),
        )));
        black_box(res);
    });
}

#[bench]
fn bench_insert_actor_stable_by_hex_string() {
    let actor = Principal::anonymous();
    FIRST_100_KEYS.iter().cloned().for_each(|k| {
        let res = black_box(insert_actor_stable_by_hex_string((
            black_box(k),
            black_box(actor),
        )));
        black_box(res);
    });
}

#[bench]
fn bench_insert_actor_stable_by_plain_string() {
    let actor = Principal::anonymous();
    FIRST_100_KEYS.iter().cloned().for_each(|k| {
        let res = black_box(insert_actor_stable_by_plain_string((
            black_box(k),
            black_box(actor),
        )));
        black_box(res);
    });
}

#[bench]
fn bench_validate_short_ok_default() {
    let res = validate::<16>(black_box(VALIDATE_SHORT_OK));
    let _ = black_box(res);
}

#[bench]
fn bench_validate_medium_ok_default() {
    let res = validate::<16>(black_box(VALIDATE_MEDIUM_OK));
    let _ = black_box(res);
}

#[bench]
fn bench_validate_long_ok_default() {
    let res = validate::<16>(black_box(VALIDATE_LONG_OK));
    let _ = black_box(res);
}

#[bench]
fn bench_validate_delim_heavy_ok_strict() {
    let res = validate_with_compiled_config::<16>(
        black_box(VALIDATE_DELIM_HEAVY_OK),
        black_box(&*CFG_HYPHEN_STRICT),
    );
    let _ = black_box(res);
}

#[bench]
fn bench_validate_error_invalid_char_default() {
    let res = validate::<16>(black_box(VALIDATE_ERROR_INVALID_CHAR));
    let _ = black_box(res);
}

#[bench]
fn bench_validate_error_consecutive_hyphen_strict() {
    let res = validate_with_compiled_config::<16>(
        black_box(VALIDATE_ERROR_CONSEC_HYPHEN),
        black_box(&*CFG_HYPHEN_STRICT),
    );
    let _ = black_box(res);
}

#[bench]
fn bench_validate_config_alnum_ok() {
    let res =
        validate_with_compiled_config::<16>(black_box("abcdef123456"), black_box(&*CFG_ALNUM));
    let _ = black_box(res);
}

#[bench]
fn bench_validate_config_permissive_mixed_ok() {
    let res = validate_with_compiled_config::<16>(
        black_box("ab-_cd-_ef-_gh"),
        black_box(&*CFG_HYPHEN_UNDERSCORE_PERMISSIVE),
    );
    let _ = black_box(res);
}

#[bench]
fn bench_validate_mixed_strict_ok() {
    let res = validate_with_compiled_config::<16>(
        black_box(VALIDATE_MIXED_STRICT_OK),
        black_box(&*CFG_HYPHEN_UNDERSCORE_STRICT),
    );
    let _ = black_box(res);
}

#[bench]
fn bench_validate_error_adjacent_mixed_strict() {
    let res = validate_with_compiled_config::<16>(
        black_box(VALIDATE_ERROR_ADJACENT_MIXED),
        black_box(&*CFG_HYPHEN_UNDERSCORE_STRICT),
    );
    let _ = black_box(res);
}
