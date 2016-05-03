fn main() {
    let mut nom = 1;
    let mut denom = 1;
    for a in 1..10 {
        for b in 1..10 {
            if a == b {
                continue;
            }
            for c in 0..10 {
                if (a * 10 + b) as f32 / (b * 10 + c) as f32 == a as f32 / c as f32 {
                    // println!("{}{}/{}{} == {}/{}", a, b, b, c, a, c);
                    nom *= a;
                    denom *= c;
                    if denom % nom == 0 {
                        denom /= nom;
                        nom = 1;
                    }
                }
            }
        }
    }
    println!("{}", denom);
}
