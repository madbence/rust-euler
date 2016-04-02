fn main() {
  let mut i = 1;
  let mut sum = 0;
  loop {
    let n = fib(i);
    println!("{}", n);
    if n >= 4000000 {
      break;
    }
    if n % 2 == 0 {
      sum += n;
    }
    i = i + 1;
  }
  println!("Sum: {}", sum);
}

fn fib(n: u32) -> u32 {
  if n < 2 {
    return n;
  }
  fib(n - 1) + fib(n - 2)
}
