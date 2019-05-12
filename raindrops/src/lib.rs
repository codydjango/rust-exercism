pub fn raindrops(n: u32) -> String {
    let mut owned = String::new();

    if n % 3 == 0 { owned.push_str("Pling") } 
    if n % 5 == 0 { owned.push_str("Plang") } 
    if n % 7 == 0 { owned.push_str("Plong") }

    return if owned.len() > 0 { owned } else { n.to_string() }
}
