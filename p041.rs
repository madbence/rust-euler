fn main() {
    let p = Permutation { current: vec![7, 6, 5, 4, 3, 2, 1] };
    for i in p {
        let mut n = 0;
        for d in i {
            n = n * 10 + d as u32;
        }
        if prime(n) {
            println!("{}", n);
            break;
        }
    }
}

struct Permutation {
    current: Vec<u8>,
}

fn swap(a: &mut Vec<u8>, i: usize, j: usize) {
    let tmp = a[i];
    a[i] = a[j];
    a[j] = tmp;
}

fn rev(mut a: &mut Vec<u8>, i: usize, j: usize) {
    for k in 0..((j - i) / 2 + 1) {
        swap(&mut a, i + k, j - k);
    }
}

impl Iterator for Permutation {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        let mut k = 0;
        let mut l = 0;
        for i in 0..(self.current.len() - 1) {
            if self.current[i] > self.current[i + 1] {
                k = i;
            }
        }
        for i in (k + 1)..(self.current.len()) {
            if self.current[k] > self.current[i] {
                l = i;
            }
        }
        if k == 0 && l == 0 {
            return None;
        }
        swap(&mut self.current, k, l);
        let last = self.current.len() - 1;
        rev(&mut self.current, k + 1, last);
        Some(self.current.clone())
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
