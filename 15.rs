fn main() {
    let mut cache: [u64; 441] = [0; 441];
    println!("Paths: {}", paths(20, 20, &mut cache));
}

fn paths(x: u32, y: u32, mut cache: &mut[u64]) -> u64 {
    let index: usize = (x * 21 + y) as usize;
    if cache[index] > 0 { return cache[index]; }
    if y == 0 { return 1; }
    if x == 0 { return 1; }
    cache[index] = (0..x + 1).fold(0, |acc, n| acc + paths(n, y - 1, &mut cache));
    cache[index]
}
