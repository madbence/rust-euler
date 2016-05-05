fn main() {
    println!("{}", (2..1_000_000).fold(0, |count, mut n| {
        for _ in 0..7 {
            if !prime(n) {
                return count;
            }
            let m = n % 10;
            n = n / 10 + m * 10u32.pow(((n as f64).log(10.0) as u32))
        }
        count + 1
    }));
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
