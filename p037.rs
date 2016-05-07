fn main() {
    let tps = TruncatablePrimes { curr: 10 };
    println!("{}", tps.take(11).fold(0, |sum, n| sum + n));
}

struct TruncatablePrimes {
    curr: u32,
}

impl Iterator for TruncatablePrimes {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        loop {
            self.curr += 1;
            let mut i = self.curr;
            let mut j = self.curr;
            while i > 0 && j > 0 && prime(i) && prime(j) {
                i /= 10;
                j = j % 10u32.pow((j as f32).log(10.0) as u32);
            }
            if i == 0 {
                return Some(self.curr);
            }
        }
    }
}

fn prime(n: u32) -> bool {
  let mut i = 3;
  if n == 1 {
      return false;
  }
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
