//! HexaURL Structure API
//!
//! This module provides the public API for the HexaUrl types using a struct-based
//! approach. It re-exports several type aliases of the internal [`HexaUrlCore`] type,
//! each representing a different fixed-size HexaUrl encoding:
//!
//! These types offer a variety of encoding options including full validation,
//! custom validation via a configuration, quick validation, and unchecked encoding
//! (which is unsafe and should be used only when the input is guaranteed to be HexaURL).
//!
//! # Usage Examples
//!
//! ```rust
//! // Using the 16-byte HexaUrl (HexaUrl16 and HexaUrl are equivalent)
//! use hexaurl::struct_api::{HexaUrl16, HexaUrl8};
//!
//! let input = "Some-User";
//! let hex16 = HexaUrl16::new(input).unwrap();
//! assert_eq!(hex16.to_string(), input.to_lowercase());
//!
//! // Using the 8-byte HexaUrl
//! let hex8 = HexaUrl8::new("Hello").unwrap();
//! assert_eq!(hex8.to_string(), "hello");
//! ```
//!
//! For additional information about encoding, decoding, and configuration options,
//! see the documentation of the underlying [`HexaUrlCore`] struct.

mod core;
#[cfg(feature = "pub-struct-core")]
#[cfg_attr(docsrs, doc(cfg(feature = "pub-struct-core")))]
pub use core::*;

/// 8-byte HexaURL:
/// Supports case-insensitive strings up to 10 characters in length.
/// Alias for internal type `HexaUrlCore<8, 10>`. See documentation for [`HexaUrlCore`].
///
/// # Examples
///
/// ```rust
/// use hexaurl::struct_api::HexaUrl8;
///
/// let input = "Some-User";
/// let hex = HexaUrl8::new(input).unwrap();
/// assert_eq!(hex.to_string(), input.to_lowercase());
/// ```
pub type HexaUrl8 = core::HexaUrlCore<8, 10>;

/// 16-byte HexaURL:
/// Supports case-insensitive strings up to 21 characters in length.
/// Same as [`HexaUrl`] but more clearly defined.
/// Alias for internal type `HexaUrlCore<16, 21>`. See documentation for [`HexaUrlCore`].
///
/// # Examples
///
/// ```rust
/// use hexaurl::struct_api::HexaUrl16;
///
/// let input = "Some-User";
/// let hex = HexaUrl16::new(input).unwrap();
/// assert_eq!(hex.to_string(), input.to_lowercase());
/// ```
pub type HexaUrl16 = core::HexaUrlCore<16, 21>;

/// 16-byte HexaURL:
/// Supports case-insensitive strings up to 21 characters in length,
/// intended as the standard general-purpose type.
/// Alias for internal type `HexaUrlCore<16, 21>`. See documentation for [`HexaUrlCore`].
///
/// # Examples
///
/// ```rust
/// use hexaurl::HexaUrl;
///
/// let input = "Some-User";
/// let hex = HexaUrl::new(input).unwrap();
/// assert_eq!(hex.to_string(), input.to_lowercase());
/// ```
pub type HexaUrl = core::HexaUrlCore<16, 21>;

/// 32-byte HexaURL:
/// Supports case-insensitive strings up to 42 characters in length.
/// Alias for internal type `HexaUrlCore<32, 42>`. See documentation for [`HexaUrlCore`].
///
/// # Examples
///
/// ```rust
/// use hexaurl::struct_api::HexaUrl32;
///
/// let input = "Some-User";
/// let hex = HexaUrl32::new(input).unwrap();
/// assert_eq!(hex.to_string(), input.to_lowercase());
/// ```
pub type HexaUrl32 = core::HexaUrlCore<32, 42>;

/// 64-byte HexaURL:
/// Supports case-insensitive strings up to 85 characters in length.
/// Alias for internal type `HexaUrlCore<64, 85>`. See documentation for [`HexaUrlCore`].
///
/// # Examples
///
/// ```rust
/// use hexaurl::struct_api::HexaUrl64;
///
/// let input = "Some-User";
/// let hex = HexaUrl64::new(input).unwrap();
/// assert_eq!(hex.to_string(), input.to_lowercase());
/// ```
pub type HexaUrl64 = core::HexaUrlCore<64, 85>;

/// 128-byte HexaURL:
/// Supports case-insensitive strings up to 170 characters in length.
/// Alias for internal type `HexaUrlCore<128, 170>`. See documentation for [`HexaUrlCore`].
///
/// # Examples
///
/// ```rust
/// use hexaurl::struct_api::HexaUrl128;
///
/// let input = "Some-User";
/// let hex = HexaUrl128::new(input).unwrap();
/// assert_eq!(hex.to_string(), input.to_lowercase());
/// ```
pub type HexaUrl128 = core::HexaUrlCore<128, 170>;

/// 256-byte HexaURL:
/// Supports case-insensitive strings up to 341 characters in length.
/// Alias for internal type `HexaUrlCore<256, 341>`. See documentation for [`HexaUrlCore`].
///
/// # Examples
///
/// ```rust
/// use hexaurl::struct_api::HexaUrl256;
///
/// let input = "Some-User";
/// let hex = HexaUrl256::new(input).unwrap();
/// assert_eq!(hex.to_string(), input.to_lowercase());
/// ```
pub type HexaUrl256 = core::HexaUrlCore<256, 341>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexaurl8() {
        let url = HexaUrl8::new("hello").unwrap();
        assert_eq!(url.to_string(), "hello");
        assert_eq!(HexaUrl8::capacity(), 10);
    }

    #[test]
    fn test_hexaurl16() {
        let url = HexaUrl16::new("hello").unwrap();
        assert_eq!(url.to_string(), "hello");
        assert_eq!(HexaUrl16::capacity(), 21);
    }

    #[test]
    fn test_hexaurl() {
        let url = HexaUrl::new("hello").unwrap();
        assert_eq!(url.to_string(), "hello");
        assert_eq!(HexaUrl::capacity(), 21);
    }

    #[test]
    fn test_hexaurl32() {
        let url = HexaUrl32::new("hello").unwrap();
        assert_eq!(url.to_string(), "hello");
        assert_eq!(HexaUrl32::capacity(), 42);
    }

    #[test]
    fn test_hexaurl64() {
        let url = HexaUrl64::new("hello").unwrap();
        assert_eq!(url.to_string(), "hello");
        assert_eq!(HexaUrl64::capacity(), 85);
    }

    #[test]
    fn test_hexaurl128() {
        let url = HexaUrl128::new("hello").unwrap();
        assert_eq!(url.to_string(), "hello");
        assert_eq!(HexaUrl128::capacity(), 170);
    }

    #[test]
    fn test_hexaurl256() {
        let url = HexaUrl256::new("hello").unwrap();
        assert_eq!(url.to_string(), "hello");
        assert_eq!(HexaUrl256::capacity(), 341);
    }
}
