fn main() {
    let mut i = 2;
    let mut n = 1;
    while divisors(n) < 500 {
        n += i;
        i += 1;
    }
    println!("{}", n);
}

fn divisors(i: u32) -> u32 {
    let mut n = i;
    let mut j = 2;
    let mut divs = 1;
    let mut terms = 1;
    let mut term = 0;
    while n > 1 {
        if !prime(j) {
            j += 1;
            continue;
        }
        if n % j == 0 {
            if term == j {
                terms += 1;
            } else {
                divs *= terms + 1;
                term = j;
                terms = 1;
            }
            n /= j;
        } else {
            j += 1;
        }
    }
    divs
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
