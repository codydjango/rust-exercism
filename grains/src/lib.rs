pub fn square(s: u32) -> u64 {
    if (s < 1 || s > 64) { panic!("Square must be between 1 and 64") }
    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1_u32..=64_u32).map(|i| square(i)).sum()
}

pub fn square(s: u32) -> u64 {
    let mut g = 1u64;

    if s < 1u32 || s > 64u32 {
        panic!("Square must be between 1 and 64");
    }

    for _ in 1..s {
        g = g * 2;
    };

    return g as u64;
}

pub fn total() -> u64 {
    let mut total = 1u64;
    let mut g = 1u64;

    for _ in 1..64 {
        g = g * 2;
        total += g;
    };

    return total;
}
