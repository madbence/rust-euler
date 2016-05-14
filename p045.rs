fn main() {
    for i in (144..).map(h) {
        if is_t(i) && is_p(i) {
            println!("{}", i);
            return;
        }
    }
}

fn h(n: u64) -> u64 {
    n * (2 * n - 1)
}

fn is_t(n: u64) -> bool {
    let b = 1f64;
    let c = -(n as f64) * 2f64;
    ((-b + (b*b - 4f64*c).sqrt()) / 2f64).fract() < 0.000001
}

fn is_p(n: u64) -> bool {
    let b = -1f64;
    let c = -(n as f64) * 2f64;
    ((-b + (b*b - 12f64*c).sqrt()) / 6f64).fract() < 0.000001
}
