fn main() {
  let mut max = 0;
  for i in 100..1000 {
    for j in i..1000 {
      if i * j < 100000 {
        continue;
      }
      if palindrome(i * j) {
        max = if i * j > max { i * j } else { max };
      }
    }
  }
  println!("Max: {}", max);
}

fn palindrome(n: u32) -> bool {
  let i = n / 1000;
  let j = i / 100 + i % 100 / 10 * 10 + i % 10 * 100;
  i * 1000 + j == n
}
