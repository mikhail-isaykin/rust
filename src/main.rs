fn main() {
    let txt: &str = "abcde";

    for chr in txt.chars().rev() {
        println!("{}", chr);
    }
}