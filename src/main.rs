fn main() {
    let num: u16 = 12345;

    let txt: &str = &num.to_string()[0..3];

    for chr in txt.chars() {
        println!("{}", chr);
    }
}