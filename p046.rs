fn main() {
    'outer: for i in 3.. {
        if i % 2 == 0 || prime(i) {
            continue;
        }
        for j in 1..(((i / 2) as f64).sqrt() + 1f64) as u32 {
            if i - 2 * j * j < 2 {
                continue;
            }
            if prime(i - 2 * j * j) {
                continue 'outer;
            }
        }
        println!("{}", i);
        break;
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
