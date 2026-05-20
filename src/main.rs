fn main() {
    let arr: [&str; 3] = ["123", "456", "789"];

    for txt in arr {
        println!("{}", txt.chars().next().unwrap())
    }
}