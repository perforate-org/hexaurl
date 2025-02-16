#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use hexaurl_config as config;
pub use hexaurl_validate as validate;
pub use hexaurl_validate::Error;

pub mod decode;
pub mod encode;
#[cfg(feature = "struct-api")]
#[cfg_attr(docsrs, doc(cfg(feature = "struct-api")))]
pub mod struct_api;

pub use decode::{decode, decode_quick_checked, decode_unchecked, decode_with_config};
pub use encode::{encode, encode_quick_checked, encode_unchecked, encode_with_config};
#[cfg(feature = "struct-api")]
#[cfg_attr(docsrs, doc(cfg(feature = "struct-api")))]
pub use struct_api::HexaUrl;

const MASK_TWO_BITS: u8 = 0b11;
const MASK_FOUR_BITS: u8 = 0b1111;
const MASK_SIX_BITS: u8 = 0b111111;
