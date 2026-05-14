fn main() {
    let txt: &str = "abcde";

    println!("{}", &txt[txt.len()-1..]);
}
