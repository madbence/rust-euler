fn main() {
  let mut s1 = 0;
  let mut s2 = 0;
  for i in 1..101 {
    s1 = s1 + i * i;
    s2 = s2 + i;
  }
  println!("{}", s2 * s2 - s1);
}
