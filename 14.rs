fn main() {
    let (len, n) = (1..1000001).fold((0, 0), |(max, m), n| {
        let len = collatz(n).count();
        if len > max {
            (len, n)
        } else {
            (max, m)
        }
    });
    println!("{}, {}", len, n);
}

fn collatz(n: u64) -> Collatz {
    Collatz { curr: n }
}

struct Collatz {
    curr: u64,
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.curr = if self.curr % 2 == 0 { self.curr / 2 } else { self.curr * 3 + 1 };
        if self.curr == 1 { None } else { Some(self.curr) }
    }
}
