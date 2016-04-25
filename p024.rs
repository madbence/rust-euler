fn main() {
    let mut p = Permutation { current: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9] };
    for d in p.nth(999998).unwrap() {
        print!("{}", d);
    }
    print!("\n");
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
        Some(self.current.clone())
    }
}
