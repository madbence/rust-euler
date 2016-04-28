fn main() {
    println!("{}", (10..1_000_000).fold(0, |sum, n| sum + if n == (0..7).fold(0, |sum, m| sum + (n / 10u32.pow(m) % 10).pow(5)) { n } else { 0 }));
}
