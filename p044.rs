fn main() {
    for (i, pi) in (1..).map(|i| (i, p(i))) {
        for (_, pj) in (1..i).map(|j| (j, p(j))) {
            if is_p(pi + pj) && is_p(pi - pj) {
                println!("{}", pi - pj);
                return;
            }
        }
    }
}

fn p(n: u64) -> u64 {
    n * (3 * n - 1) / 2
}

fn is_p(n: u64) -> bool {
    let b = 1f32;
    let c = -(n as f32) * 2f32;
    ((b + (b*b - 12f32*c).sqrt()) / 6f32).fract() < 0.0001
}
