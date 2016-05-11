use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("p042_words.txt").unwrap();
    let mut s = String::new();

    f.read_to_string(&mut s).unwrap();
    println!("{}", s.split(",").filter(|word| triangle(score(word))).count());
}

fn triangle(n: u32) -> bool {
    let mut i = 1;
    loop {
        let j = i * (i + 1) / 2;
        if j == n {
            return true;
        }
        if j > n {
            return false;
        }
        i += 1;
    }
}

fn score(word: &str) -> u32 {
    word.as_bytes().iter().fold(0, |sum, &c| sum + match c {
        65...90 => c as u32 - 64,
        _ => 0
    })
}

