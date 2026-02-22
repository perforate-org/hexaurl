const PAIR_MASK: u64 = 0x000000FF000000FF;
const BIAS: u64 = 0x8000000080000000;
const BYTE_HIGH_BITS: u64 = 0x8080808080808080;
const BYTE_ONES: u64 = 0x0101010101010101;

#[inline(always)]
fn validate_pair_alnum(pair: u64) -> u64 {
    // Check 'a'..'z'
    let lower = pair | 0x0000002000000020;
    // (val | BIAS) - 'a'. If val < 'a', borrow -> Bit 0. If val >= 'a', Bit 1.
    let l_ge_a = (lower | BIAS).wrapping_sub(0x0000006100000061);
    // ('z' | BIAS) - val. If val > 'z', borrow -> Bit 0. If val <= 'z', Bit 1.
    let l_le_z = (0x0000007A0000007A_u64 | BIAS).wrapping_sub(lower);
    let letter_ok = (l_ge_a & l_le_z) & BIAS;

    // Check '0'..'9'
    let d_ge_0 = (pair | BIAS).wrapping_sub(0x0000003000000030);
    let d_le_9 = (0x0000003900000039_u64 | BIAS).wrapping_sub(pair);
    let digit_ok = (d_ge_0 & d_le_9) & BIAS;

    letter_ok | digit_ok
}

#[inline(always)]
fn pair_is_dash(pair: u64) -> u64 {
    let dash_xor = pair ^ 0x0000002D0000002D;
    let dash_check = (dash_xor | BIAS).wrapping_sub(0x0000000100000001);
    (!dash_check) & BIAS
}

#[inline(always)]
fn pair_is_underscore(pair: u64) -> u64 {
    let under_xor = pair ^ 0x0000005F0000005F;
    let under_check = (under_xor | BIAS).wrapping_sub(0x0000000100000001);
    (!under_check) & BIAS
}

#[inline(always)]
fn has_byte(chunk: u64, needle: u8) -> bool {
    let x = chunk ^ (u64::from(needle) * BYTE_ONES);
    ((x.wrapping_sub(BYTE_ONES)) & (!x) & BYTE_HIGH_BITS) != 0
}

#[inline(always)]
fn split_pairs(chunk: u64) -> (u64, u64, u64, u64) {
    let pair1 = chunk & PAIR_MASK;
    let pair2 = (chunk >> 16) & PAIR_MASK;
    let pair3 = (chunk >> 8) & PAIR_MASK;
    let pair4 = (chunk >> 24) & PAIR_MASK;
    (pair1, pair2, pair3, pair4)
}

#[inline(always)]
pub fn validate_chunk_alnum(chunk: u64) -> (bool, bool, bool) {
    let (pair1, pair2, pair3, pair4) = split_pairs(chunk);
    let v1 = validate_pair_alnum(pair1);
    let v2 = validate_pair_alnum(pair2);
    let v3 = validate_pair_alnum(pair3);
    let v4 = validate_pair_alnum(pair4);
    ((v1 & v2 & v3 & v4) == BIAS, false, false)
}

#[inline(always)]
pub fn validate_chunk_hyphen(chunk: u64) -> (bool, bool, bool) {
    let (pair1, pair2, pair3, pair4) = split_pairs(chunk);
    let v1 = validate_pair_alnum(pair1) | pair_is_dash(pair1);
    let v2 = validate_pair_alnum(pair2) | pair_is_dash(pair2);
    let v3 = validate_pair_alnum(pair3) | pair_is_dash(pair3);
    let v4 = validate_pair_alnum(pair4) | pair_is_dash(pair4);
    ((v1 & v2 & v3 & v4) == BIAS, has_byte(chunk, b'-'), false)
}

#[inline(always)]
pub fn validate_chunk_underscore(chunk: u64) -> (bool, bool, bool) {
    let (pair1, pair2, pair3, pair4) = split_pairs(chunk);
    let v1 = validate_pair_alnum(pair1) | pair_is_underscore(pair1);
    let v2 = validate_pair_alnum(pair2) | pair_is_underscore(pair2);
    let v3 = validate_pair_alnum(pair3) | pair_is_underscore(pair3);
    let v4 = validate_pair_alnum(pair4) | pair_is_underscore(pair4);
    ((v1 & v2 & v3 & v4) == BIAS, false, has_byte(chunk, b'_'))
}

/// Validates an 8-byte chunk for alnum + '-' + '_'.
/// Returns (is_valid, has_hyphen, has_underscore).
#[inline(always)]
pub fn validate_chunk_both(chunk: u64) -> (bool, bool, bool) {
    let (pair1, pair2, pair3, pair4) = split_pairs(chunk);
    let v1 = validate_pair_alnum(pair1) | pair_is_dash(pair1) | pair_is_underscore(pair1);
    let v2 = validate_pair_alnum(pair2) | pair_is_dash(pair2) | pair_is_underscore(pair2);
    let v3 = validate_pair_alnum(pair3) | pair_is_dash(pair3) | pair_is_underscore(pair3);
    let v4 = validate_pair_alnum(pair4) | pair_is_dash(pair4) | pair_is_underscore(pair4);
    (
        (v1 & v2 & v3 & v4) == BIAS,
        has_byte(chunk, b'-'),
        has_byte(chunk, b'_'),
    )
}
