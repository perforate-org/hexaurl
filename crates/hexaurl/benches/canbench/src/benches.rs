extern crate test;

use super::*;
use canbench_rs::bench;
use once_cell::sync::Lazy;
use test::black_box;

const INPUT: &str = "hello-world";
static INPUT_ENCODED: Lazy<HexaUrl> = Lazy::new(|| HexaUrl::new(black_box(INPUT)).unwrap());
static FIRST_100_KEYS: Lazy<Vec<String>> =
    Lazy::new(|| MAP_KEYS.iter().take(100).map(|k| k.to_string()).collect());
static FIRST_100_HEX: Lazy<Vec<HexaUrl>> = Lazy::new(|| {
    FIRST_100_KEYS
        .iter()
        .map(|k| HexaUrl::new(k).unwrap())
        .collect()
});

#[bench]
fn bench_encode() {
    let res = HexaUrl::new(black_box(INPUT)).unwrap();
    black_box(res);
}

#[bench]
fn bench_encode_quick_checked() {
    let res = HexaUrl::new_quick_checked(black_box(INPUT)).unwrap();
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
