fn main() {
    let mut sum = 0;
    'outer: for n in 1000..10000 {
        let h0 = hash(n);
        if !part(&h0) {
            continue;
        }
        for m in 1..(n / 2) {
            if n % m != 0 {
                continue;
            }
            if n / m > m {
                continue;
            }
            let mut h1 = hash(m);
            let h2 = hash(n / m);
            add(&mut h1, &h0);
            add(&mut h1, &h2);
            if ma(&h1) {
                // println!("{}={}x{} {:?}", n, m, n / m, h1);
                sum += n;
                continue 'outer;
            }
        }
    }
    println!("{}", sum);
}

fn add<'a>(a: &'a mut[u8], b: &[u8]) -> &'a [u8] {
    for i in 0..(b.len()) {
        a[i] += b[i];
    }
    a
}

fn ma(a: &[u8]) -> bool {
    let pattern = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    for i in 0..10 {
        if a[i] != pattern[i] {
            return false;
        }
    }
    true
}

fn part(a: &[u8]) -> bool {
    a[0] == 0 && a.iter().all(|&c| c < 2)
}

fn hash(mut a: u32) -> [u8; 10] {
    let mut h: [u8; 10] = [0; 10];
    loop {
        if a == 0 {
            break;
        }
        h[(a % 10) as usize] += 1;
        a = a / 10;
    }
    h
}
