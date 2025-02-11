mod core;

pub type HexaUrl8 = core::HexaUrl<8, 10>;
pub type HexaUrl16 = core::HexaUrl<16, 21>;
pub type HexaUrl = core::HexaUrl;
pub type HexaUrl32 = core::HexaUrl<32, 42>;
pub type HexaUrl64 = core::HexaUrl<64, 85>;
pub type HexaUrl128 = core::HexaUrl<128, 170>;
pub type HexaUrl256 = core::HexaUrl<256, 341>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexaurl8() {
        let url = HexaUrl8::new("hello").unwrap();
        assert_eq!(url.to_string(), "hello");
        assert_eq!(HexaUrl8::max_len(), 10);
    }

    #[test]
    fn test_hexaurl16() {
        let url = HexaUrl16::new("hello").unwrap();
        assert_eq!(url.to_string(), "hello");
        assert_eq!(HexaUrl16::max_len(), 21);
    }

    #[test]
    fn test_hexaurl() {
        let url = HexaUrl::new("hello").unwrap();
        assert_eq!(url.to_string(), "hello");
        assert_eq!(HexaUrl::max_len(), 21);
    }

    #[test]
    fn test_hexaurl32() {
        let url = HexaUrl32::new("hello").unwrap();
        assert_eq!(url.to_string(), "hello");
        assert_eq!(HexaUrl32::max_len(), 42);
    }

    #[test]
    fn test_hexaurl64() {
        let url = HexaUrl64::new("hello").unwrap();
        assert_eq!(url.to_string(), "hello");
        assert_eq!(HexaUrl64::max_len(), 85);
    }

    #[test]
    fn test_hexaurl128() {
        let url = HexaUrl128::new("hello").unwrap();
        assert_eq!(url.to_string(), "hello");
        assert_eq!(HexaUrl128::max_len(), 170);
    }

    #[test]
    fn test_hexaurl256() {
        let url = HexaUrl256::new("hello").unwrap();
        assert_eq!(url.to_string(), "hello");
        assert_eq!(HexaUrl256::max_len(), 341);
    }
}
