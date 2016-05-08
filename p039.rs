fn main() {
    let mut max = (0, 0);
    for p in 3..1001 {
        let mut sols = 0;
        for a in 2..p {
            for b in 1..a {
                let c = p - a - b;
                if c < 0 {
                    continue;
                }
                if p == a + b + c && a * a == b * b + c * c {
                    sols += 1;
                }
            }
        }
        if sols > max.0 {
            max = (sols, p);
        }
    }
    println!("{}", max.1);
}
