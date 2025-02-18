//! HexaURL utilities

/// Byte size threshold for switching between linear and binary search.
const THRESHOLD: usize = 16;

/// Search for the first zero byte in a byte array.
///
/// O(log N)
#[inline(always)]
pub fn len<const N: usize>(bytes: &[u8; N]) -> usize {
    if N <= THRESHOLD {
        linear_search(bytes, N)
    } else {
        binary_search(bytes)
    }
}

/// Linear search to find first 0 byte.
///
/// O(N)
#[inline(always)]
fn linear_search(bytes: &[u8], len: usize) -> usize {
    let midpoint = len / 2;

    if bytes[midpoint] == 0 {
        // Search left from midpoint
        for i in (0..midpoint).rev() {
            if bytes[i] != 0 {
                return i + 1;
            }
        }
        0
    } else {
        // Search right from midpoint
        for (i, &byte) in bytes.iter().enumerate().take(len).skip(midpoint + 1) {
            if byte == 0 {
                return i;
            }
        }
        len
    }
}

/// Binary search to find first 0 byte.
///
/// O(log N)
#[inline(always)]
fn binary_search<const N: usize>(bytes: &[u8; N]) -> usize {
    let (mut left, mut right) = (0, N);

    while left < right {
        if right - left <= THRESHOLD {
            return left + linear_search(&bytes[left..right], right - left);
        }

        let mid = left + (right - left) / 2;
        if bytes[mid] == 0 {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len_all_nonzero() {
        // Array with no zero bytes: should return the array length.
        let arr = [1u8; 100];
        assert_eq!(len(&arr), 100);
    }

    #[test]
    fn test_len_with_zero() {
        // Array with a zero in the middle: should return the index of the first 0.
        let mut arr = [1u8; 100];
        arr[50] = 0;
        assert_eq!(len(&arr), 50);
    }

    #[test]
    fn test_len_zero_at_start() {
        // Array where the first element is 0.
        let arr = [0u8; 100];
        assert_eq!(len(&arr), 0);
    }

    #[test]
    fn test_len_zero_at_end() {
        // Array where only the last element is 0.
        let mut arr = [1u8; 100];
        arr[99] = 0;
        assert_eq!(len(&arr), 99);
    }
}
