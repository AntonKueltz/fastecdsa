#[inline]
pub fn select_mask(a: usize, b: usize) -> u64 {
    let diff = (a as i64) ^ (b as i64);
    let is_nonzero = (diff | diff.wrapping_neg()) >> 63;

    !(is_nonzero as u64)
}

#[inline]
pub fn test_bit(n: &[u8], j: usize) -> bool {
    let byte = j >> 3;
    let bit = j & 0b111;

    ((n.get(byte).unwrap_or(&0u8) >> bit) & 1) == 1
}
