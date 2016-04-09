fn main() {
    let mut a = Vec::new();
    a.push(2);

    for _ in 1..400 {
        a.push(0);
    }

    for _ in 1..1000 {
        a = add(&a, &a);
    }
    println!("{}", a.iter().fold(0, |sum, &n| sum + n as u32));
}

fn add(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.iter().zip(b.iter()).scan(0, |c, (a, b)| {
        let s = a + b + *c;
        *c = s / 10;
        Some(s % 10)
    }).collect()
}
