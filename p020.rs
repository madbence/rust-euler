fn main() {
    let a = fact(100);
    println!("{}", a.iter().fold(0, |sum, &n| sum + n as u32));
}

fn fact(n: u8) -> Vec<u8> {
    let mut a = Vec::new();
    a.push(1);

    for _ in 1..400 {
        a.push(0);
    }

    (2..(n + 1)).fold(a, |prod, i| mul(&prod, i))
}

fn mul(a: &Vec<u8>, b: u8) -> Vec<u8> {
    (1..b).fold(a.clone(), |sum, _| add(&sum, a))
}

fn add(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.iter().zip(b.iter()).scan(0, |c, (a, b)| {
        let s = a + b + *c;
        *c = s / 10;
        Some(s % 10)
    }).collect()
}
