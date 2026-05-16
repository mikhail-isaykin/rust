fn main() {
    let txt: &str = "abcde";

    for char in txt.chars() {
        println!("{}", char);
    }
}