fn main() {
    let mut max = 0;
    let mut prod = 0;
    for a in -999..1000 {
        for b in -999..1000 {
            let c = ((0 as i32)..).take_while(|&n| prime((n * n + a * n + b) as u32)).count();
            if c > max {
                max = c;
                prod = a * b;
            }
        }
    }
    println!("{}", prod);
}

fn prime(n: u32) -> bool {
    let mut i = 3;
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
