fn main() {
  let mut i = 2;
  let mut j = 0;
  loop {
    if prime(i) {
      j = j + 1;
    }
    if j == 10001 {
      println!("{}", i);
      break;
    }
    i = i + 1;
  }
}

fn prime(i: u32) -> bool {
  for j in 2..i/2+1 {
    if i % j == 0 {
      return false;
    }
  }
  return true;
}
