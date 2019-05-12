pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2u32;
    } else if n == 1 {
        return 3u32;
    }

    // sieve of eratosthenes
    let n = (n + 1) as f32;
    let limit = (3f32 + (n * n.ln()) + (n * n.ln().ln())).ceil() as usize;

    let mut sieve: Vec<bool> = vec![true; limit];
    let mut count = 0;

    for i in 2usize..limit {
        if !sieve[i] { continue }
        
        count += 1;
        
        if count == n as u32 { return i as u32; }
        for mult in ((i*i)..limit).step_by(i) { sieve[mult] = false; }
    }

    panic!("prime not found");
}
