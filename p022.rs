use std::fs::File;
use std::io::prelude::*;


fn main() {
    let mut f = File::open("p022_names.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let mut names = s.split(",").collect::<Vec<&str>>();
    names.sort();
    let sum = names.iter().zip(1..6000).fold(0, |sum, (name, pos)| sum + pos * score(name));

    println!("{}", sum);
}

fn score(name: &str) -> u32 {
    name.as_bytes().iter().fold(0, |sum, &c| sum + match c {
        65...90 => c as u32 - 64,
        _ => 0
    })
}
