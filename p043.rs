fn main() {
    let mut sum = 0;
    for n in permutations(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]) {
        if (n / 100_000_000 % 10 * 100 + n / 10_000_000 % 10 * 10 + n / 1_000_000 % 10) % 2 == 0 &&
            (n / 10_000_000 % 10 * 100 + n / 1_000_000 % 10 * 10 + n / 100_000 % 10) % 3 == 0 &&
            (n / 1_000_000 % 10 * 100 + n / 100_000 % 10 * 10 + n / 10_000 % 10) % 5 == 0 &&
            (n / 100_000 % 10 * 100 + n / 10_000 % 10 * 10 + n / 1_000 % 10) % 7 == 0 &&
            (n / 10_000 % 10 * 100 + n / 1_000 % 10 * 10 + n / 100 % 10) % 11 == 0 &&
            (n / 1_000 % 10 * 100 + n / 100 % 10 * 10 + n / 10 % 10) % 13 == 0 &&
            (n / 100 % 10 * 100 + n / 10 % 10 * 10 + n % 10) % 17 == 0 {
                sum += n;
        }
    }
    println!("{}", sum);
}

fn permutations(v: Vec<u8>) -> Permutation {
    Permutation { current: v }
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
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let mut k = 0;
        let mut l = 0;
        for i in 0..(self.current.len() - 1) {
            if self.current[i] < self.current[i + 1] {
                k = i;
            }
        }
        for i in (k + 1)..(self.current.len()) {
            if self.current[k] < self.current[i] {
                l = i;
            }
        }
        if k == 0 && l == 0 {
            return None;
        }
        swap(&mut self.current, k, l);
        let last = self.current.len() - 1;
        rev(&mut self.current, k + 1, last);
        Some(self.current.iter().fold(0u64, |prod, &n| prod * 10 + n as u64))
    }
}
