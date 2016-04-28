fn main() {
    let mut max = (0, 0);
    for n in 1..1000 {
        let mut i = 1;
        let mut hit: [u32; 1000] = [0; 1000];
        // print!("{}: 0.", n);
        let mut len = 0;
        while i > 0 && hit[i] == 0 {
            hit[i] = 1;
            while n > i { i *= 10; }
            // print!("{}", i / n);
            i = i % n;
            len += 1;
        }
        if len > max.0 {
            max = (len, n)
        }
        // print!("\n");
    }
    println!("{}", max.1);
}
