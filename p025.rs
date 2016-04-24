fn main() {
    let mut a = Vec::new();
    let mut b = Vec::new();
    a.push(1);
    b.push(1);
    for _ in 1..1100 {
        a.push(0);
        b.push(0);
    }

    let f = Fib { a: a, b: b };
    let mut i = 3;
    for n in f {
        let len = n.iter().rev().skip_while(|&a| *a == 0).count();
        if len >= 1000 {
            println!("{}", i);
            break;
        }
        i += 1;
    }
}

struct Fib {
    a: Vec<u8>,
    b: Vec<u8>,
}

impl Iterator for Fib {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Vec<u8>> {
        let n = add(&self.a, &self.b);
        self.a = self.b.clone();
        self.b = n;
        Some(self.b.clone())
    }
}

fn add(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.iter().zip(b.iter()).scan(0, |c, (a, b)| {
        let s = a + b + *c;
        *c = s / 10;
        Some(s % 10)
    }).collect()
}
