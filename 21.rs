fn main() {
    let ds: Vec<u32> = (0..10000).map(|n| d(n)).collect();
    let mut c = 0;
    for a in 1..10000 {
        for b in (a + 1)..10000 {
            if ds[a as usize] == b && ds[b as usize] == a {
                c += a + b;
            }
        }
    }
    println!("{}", c);
}

fn d(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }
    (1..n).fold(0, |sum, i| sum + if n % i == 0 { i } else { 0 })
}
