fn main() {
    let (sum, _, _) = (1..(1001 * 1001 + 1)).fold((0, 2, 0), |(sum, m, c), n| (sum + if c % m == 0 { n } else { 0 }, m + if c >= m * 4 { 2 } else { 0 }, if c >= m * 4 { 1 } else { c + 1 }));
    println!("{}", sum);
}
