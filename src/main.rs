fn main() {
    let word1: &str = "abc";
    let word2: &str = "ade";

    if &word1[..1] == &word2[..1] {
        println!("Первые буквы совпадают");
    }
    else {
        println!("Первые буквы не совпадают");
    }
}
