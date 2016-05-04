fn main() {
    println!("{}", (3..).take(100_000).fold(0, |sum, n| {
        let mut m = n;
        let mut s = 0;
        while m > 0 {
            s += fac(m % 10);
            m /= 10;
        }
        if s == n {
            sum + n
        } else {
            sum
        }
    }));
}

fn fac(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    (1..n + 1).fold(1, |prod, n| prod * n)
}
