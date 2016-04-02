fn main() {
  let mut n = 600851475143;
  let mut i = 3;
  loop {
    if i > n {
      break;
    }
    if n % i == 0 && prime(i) {
      println!("{} / {} = {}", n, i, n / i);
      n = n / i;
      continue;
    }
    i = i + 2;
  }
}

fn prime(n: u64) -> bool {
  for i in 2..n {
    if n % i == 0 {
      return false;
    }
  }
  true
}
