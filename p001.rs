fn main() {
  println!("{}", (1..1000).fold(0, |sum, n| sum + if n % 3 == 0 || n % 5 == 0 { n } else { 0 }));
}
