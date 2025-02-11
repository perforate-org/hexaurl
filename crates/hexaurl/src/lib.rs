#![doc = include_str!("../README.md")]

pub use hexaurl_validate::Error;
pub use hexaurl_config as config;
pub use hexaurl_validate as validate;

pub mod encode;
pub mod decode;
#[cfg(feature = "with-struct")]
pub mod struct_api;

pub use encode::{encode, encode_unchecked};
pub use decode::{decode, decode_unchecked};
#[cfg(feature = "with-struct")]
pub use struct_api::HexaUrl;

const MASK_TWO_BITS: u8 = 0b11;
const MASK_FOUR_BITS: u8 = 0b1111;
const MASK_SIX_BITS: u8 = 0b111111;
const SHIFT_TWO_BITS: u8 = 2;
const SHIFT_FOUR_BITS: u8 = 4;
const SHIFT_SIX_BITS: u8 = 6;
const ASCII_OFFSET: u8 = 32;
