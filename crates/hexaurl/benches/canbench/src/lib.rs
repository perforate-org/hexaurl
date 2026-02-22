#![cfg_attr(feature = "nightly", feature(test))]

use candid::Principal;
use hexaurl::{HexaUrl, validate::validate};
use ic_cdk_macros::*;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    {DefaultMemoryImpl, StableBTreeMap},
};
use once_cell::sync::Lazy;
use std::{cell::RefCell, collections::BTreeMap};

#[cfg(feature = "nightly")]
#[cfg(feature = "canbench-rs")]
mod benches;

type Memory = VirtualMemory<DefaultMemoryImpl>;

static MAP_KEYS: Lazy<Vec<&str>> = Lazy::new(|| {
    include_str!("../../list.txt")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect()
});

struct HeapActors {
    by_hex: BTreeMap<HexaUrl, Principal>,
    by_plain: BTreeMap<String, Principal>,
}

impl HeapActors {
    fn new() -> Self {
        let mut by_hex = BTreeMap::new();
        let mut by_plain = BTreeMap::new();

        for key in MAP_KEYS.iter().copied() {
            let lower = key.to_ascii_lowercase();
            let hex =
                HexaUrl::new(key).expect("pre-generated list must contain only valid HexaUrl keys");
            by_hex.insert(hex, Principal::anonymous());
            by_plain.insert(lower, Principal::anonymous());
        }

        Self { by_hex, by_plain }
    }
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static STABLE_ACTORS_HEXAURL: RefCell<StableBTreeMap<HexaUrl, Principal, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    pub static STABLE_ACTORS_STRING: RefCell<StableBTreeMap<String, Principal, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );

    pub static HEAP_ACTORS: RefCell<HeapActors> = RefCell::new(HeapActors::new());
}

#[query]
fn get_actor_by_hex(input: HexaUrl) -> Option<Principal> {
    HEAP_ACTORS.with(|actors| actors.borrow().by_hex.get(&input).copied())
}

#[query]
fn get_actor_by_hex_string(input: String) -> Option<Principal> {
    let key = HexaUrl::new_quick(&input).ok()?;
    HEAP_ACTORS.with(|actors| actors.borrow().by_hex.get(&key).copied())
}

#[query]
fn get_actor_by_plain_string(input: String) -> Option<Principal> {
    HEAP_ACTORS.with(|actors| actors.borrow().by_plain.get(&input).copied())
}

#[query]
fn get_actor_stable_by_hex(input: HexaUrl) -> Option<Principal> {
    STABLE_ACTORS_HEXAURL.with(|actors| actors.borrow().get(&input))
}

#[query]
fn get_actor_stable_by_hex_string(input: String) -> Option<Principal> {
    let key = HexaUrl::new_quick(&input).ok()?;
    STABLE_ACTORS_HEXAURL.with(|actors| actors.borrow().get(&key))
}

#[query]
fn get_actor_stable_by_plain_string(input: String) -> Option<Principal> {
    STABLE_ACTORS_STRING.with(|actors| actors.borrow().get(&input))
}

#[update]
fn insert_actor_by_hex(input: (HexaUrl, Principal)) -> Option<Principal> {
    HEAP_ACTORS.with(|actors| actors.borrow_mut().by_hex.insert(input.0, input.1))
}

#[update]
fn insert_actor_by_hex_string(input: (String, Principal)) -> Option<Principal> {
    let key = HexaUrl::new(&input.0).ok()?;
    HEAP_ACTORS.with(|actors| actors.borrow_mut().by_hex.insert(key, input.1))
}

#[update]
fn insert_actor_by_plain_string(input: (String, Principal)) -> Option<Principal> {
    validate::<16>(&input.0).ok()?;
    HEAP_ACTORS.with(|actors| actors.borrow_mut().by_plain.insert(input.0, input.1))
}

#[update]
fn insert_actor_stable_by_hex(input: (HexaUrl, Principal)) -> Option<Principal> {
    STABLE_ACTORS_HEXAURL.with(|actors| actors.borrow_mut().insert(input.0, input.1))
}

#[update]
fn insert_actor_stable_by_hex_string(input: (String, Principal)) -> Option<Principal> {
    let key = HexaUrl::new(&input.0).ok()?;
    STABLE_ACTORS_HEXAURL.with(|actors| actors.borrow_mut().insert(key, input.1))
}

#[update]
fn insert_actor_stable_by_plain_string(input: (String, Principal)) -> Option<Principal> {
    validate::<16>(&input.0).ok()?;
    STABLE_ACTORS_STRING.with(|actors| actors.borrow_mut().insert(input.0, input.1))
}

ic_cdk::export_candid!();
