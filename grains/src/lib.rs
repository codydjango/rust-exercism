
pub fn square(s: u32) -> u64 {
    if s < 1u32 || s > 64u32 { panic!("Square must be between 1 and 64"); }
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1u32..=64u32).map(|i| square(i)).sum::<u64>()
}
