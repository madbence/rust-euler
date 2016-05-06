fn main() {
    println!("{}", (1..1_000_000).fold(0, |sum, n| {
        if palindrome(&to_string(n, 10)) && palindrome(&to_string(n, 2)) {
            sum + n
        } else {
            sum
        }
    }));
}

fn palindrome(s: &String) -> bool {
    s.chars().zip(s.chars().rev()).all(|(a, b)| a == b)
}

fn to_string(mut n: u32, base: u32) -> String {
    let mut s = String::new();
    while n > 0 {
        s.push_str((n % base).to_string().as_str());
        n = n / base;
    }
    s
}
