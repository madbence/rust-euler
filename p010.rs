fn main() {
  let mut sum: u64 = 0;
  for i in 2..2000000 {
    if prime(i) {
      sum += i as u64;
    }
  }
  println!("{}", sum);
}

fn prime(n: u32) -> bool {
  let mut i = 3;
  if n < 4 {
    return true;
  }
  if n % 2 == 0 {
    return false;
  }
  while (i as f64) < (n as f64).sqrt() + 1. {
    if n % i == 0 {
      return false;
    }
    i += 2;
  }
  true
}
