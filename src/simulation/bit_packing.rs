pub fn bits_needed(n: u32) -> u32 {
    32 - n.leading_zeros()
}