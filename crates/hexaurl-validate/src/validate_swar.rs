const PAIR_MASK: u64 = 0x000000FF000000FF;
const BIAS: u64 = 0x8000000080000000;

#[inline(always)]
fn validate_pair(pair: u64, allow_hyphen: bool, allow_underscore: bool) -> u64 {
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

    let mut valid = letter_ok | digit_ok;

    if allow_hyphen {
        // Check '-'
        // (val ^ '-') | BIAS - 1.
        // If val == '-', xor 0. 0|BIAS - 1 = 7F... (Bit 0).
        // If val != '-', xor > 0. >0|BIAS - 1 = 80... (Bit 1).
        // We want Equal -> 1.
        let dash_xor = pair ^ 0x0000002D0000002D;
        let dash_check = (dash_xor | BIAS).wrapping_sub(0x0000000100000001);
        // dash_check & BIAS is 0 if Equal, 1 if Not Equal.
        // We want 1 if Equal.
        let dash_ok = (!dash_check) & BIAS;
        valid |= dash_ok;
    }

    if allow_underscore {
        // Check '_'
        let under_xor = pair ^ 0x0000005F0000005F;
        let under_check = (under_xor | BIAS).wrapping_sub(0x0000000100000001);
        let under_ok = (!under_check) & BIAS;
        valid |= under_ok;
    }

    valid
}

/// Validates an 8-byte chunk.
/// Returns (is_valid, has_hyphen, has_underscore).
#[inline(always)]
pub fn validate_chunk(
    chunk: u64,
    allow_hyphen: bool,
    allow_underscore: bool,
) -> (bool, bool, bool) {
    let pair1 = chunk & PAIR_MASK;
    let pair2 = (chunk >> 16) & PAIR_MASK;
    let pair3 = (chunk >> 8) & PAIR_MASK;
    let pair4 = (chunk >> 24) & PAIR_MASK;

    let v1 = validate_pair(pair1, allow_hyphen, allow_underscore);
    let v2 = validate_pair(pair2, allow_hyphen, allow_underscore);
    let v3 = validate_pair(pair3, allow_hyphen, allow_underscore);
    let v4 = validate_pair(pair4, allow_hyphen, allow_underscore);

    let all_valid = (v1 & v2 & v3 & v4) == BIAS;

    // Determine if hyphens/underscores present?
    // This requires checking specific conditions again or outputting them from validate_pair.
    // To save perf, we can just return valid/invalid for now.
    // If we need has_hyphen, we can recalc or return from validate_pair.
    // But existing lib.rs code uses has_hyphen |= h;
    // So we need 'h'.

    // Re-calc simple has_hyphen logic?
    // pair ^ '-' == 0.
    // has_h = ((chunk ^ 0x2D...) has zero byte).
    // has_zero_byte(x) = (x - 0x01...) & ~x & 0x80...
    // This is standard SWAR. Can apply to full u64!

    let has_hyphen = if allow_hyphen {
        let x = chunk ^ 0x2D2D2D2D2D2D2D2D;
        ((x.wrapping_sub(0x0101010101010101)) & (!x) & 0x8080808080808080) != 0
    } else {
        false
    };

    let has_underscore = if allow_underscore {
        let x = chunk ^ 0x5F5F5F5F5F5F5F5F;
        ((x.wrapping_sub(0x0101010101010101)) & (!x) & 0x8080808080808080) != 0
    } else {
        false
    };

    (all_valid, has_hyphen, has_underscore)
}
