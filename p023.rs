fn main() {
    let abundants: Vec<bool> = (0..30000).map(|n| n < (1..n).fold(0, |sum, i| sum + if n % i == 0 { i } else { 0 })).collect();
    println!("{}", (1..30000).fold(0, |sum, n| sum + if (1..n).any(|i| abundants[i as usize] && abundants[(n - i) as usize]) { 0 } else { n }));
}
