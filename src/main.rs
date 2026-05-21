fn main() {
    let num: u16 = 12345;

    for chr in num.to_string().chars().rev() {
        println!("{}", chr);
    }
}