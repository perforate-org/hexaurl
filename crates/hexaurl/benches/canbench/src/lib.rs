#![cfg_attr(feature = "nightly", feature(test))]

use candid::Principal;
use hexaurl::{validate::validate, HexaUrl};
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

    pub static ACTORS_HEXAURL: RefCell<BTreeMap<HexaUrl, Principal>> = const { RefCell::new(BTreeMap::new()) };
    pub static ACTORS_STRING: RefCell<BTreeMap<String, Principal>> = const { RefCell::new(BTreeMap::new()) };
}

#[init]
fn init() {
    for key in MAP_KEYS.iter().copied() {
        let str = key.to_ascii_lowercase();
        let hex = HexaUrl::new(key).unwrap();
        ACTORS_HEXAURL.with(|actors| actors.borrow_mut().insert(hex, Principal::anonymous()));
        ACTORS_STRING.with(|actors| {
            actors
                .borrow_mut()
                .insert(str.clone(), Principal::anonymous())
        });
        STABLE_ACTORS_HEXAURL
            .with(|actors| actors.borrow_mut().insert(hex, Principal::anonymous()));
        STABLE_ACTORS_STRING.with(|actors| actors.borrow_mut().insert(str, Principal::anonymous()));
    }
}

#[query]
fn get_actor_by_hex(input: HexaUrl) -> Option<Principal> {
    ACTORS_HEXAURL.with(|actors| actors.borrow().get(&input).copied())
}

#[query]
fn get_actor_by_hex_string(input: String) -> Option<Principal> {
    let key = HexaUrl::new_quick(&input).ok()?;
    ACTORS_HEXAURL.with(|actors| actors.borrow().get(&key).copied())
}

#[query]
fn get_actor_by_plain_string(input: String) -> Option<Principal> {
    ACTORS_STRING.with(|actors| actors.borrow().get(&input).copied())
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
    ACTORS_HEXAURL.with(|actors| actors.borrow_mut().insert(input.0, input.1))
}

#[update]
fn insert_actor_by_hex_string(input: (String, Principal)) -> Option<Principal> {
    let key = HexaUrl::new(&input.0).ok()?;
    ACTORS_HEXAURL.with(|actors| actors.borrow_mut().insert(key, input.1))
}

#[update]
fn insert_actor_by_plain_string(input: (String, Principal)) -> Option<Principal> {
    validate::<16>(&input.0).ok()?;
    ACTORS_STRING.with(|actors| actors.borrow_mut().insert(input.0, input.1))
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
