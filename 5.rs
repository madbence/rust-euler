fn main() {
  let mut i = 1;
  'outer: loop {
    for j in 2..21 {
      if i % j != 0 {
        i = i + 1;
        continue 'outer;
      }
    }
    println!("{}", i);
    break;
  }
}
