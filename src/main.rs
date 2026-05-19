fn main() {
    let mut num: u32 = 10;
    let mut exp: u32 = 1;

    while num != 10000 {
        num *= 10;
        exp += 1
    }

    println!("{}", exp);
}