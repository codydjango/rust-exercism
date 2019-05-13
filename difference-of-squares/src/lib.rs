// originally had:
// (1u32..=n).collect::<Vec<u32>>().iter().sum::<u32>().pow(2)
// simplified to:
pub fn square_of_sum(n: u32) -> u32 {
    (1u32..=n).sum::<u32>().pow(2)
}

// originally had:
// (1u32..=n).collect::<Vec<u32>>().iter().map(|i| { i.pow(2) }).sum()
// simplified to:
pub fn sum_of_squares(n: u32) -> u32 {
    (1u32..=n).map(|i| i.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
