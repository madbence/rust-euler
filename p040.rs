fn main() {
    println!("{}", (1..1_000_001).flat_map(|j| j.to_string().chars().collect::<Vec<_>>()).zip(1..).filter(|&(_, n)| match n {
        1|10|100|1_000|10_000|100_000|1_000_000 => true,
        _ => false
    }).fold(1, |prod, (i, _)| prod * i.to_digit(10).unwrap()));
}
