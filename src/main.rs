use std::io;

fn write (word: &mut String) -> () {
    io::stdin()
        .read_line(word)
        .unwrap();
}

fn chr (word: &str, flag: &str) -> char {
    if flag == "first" {
        word.trim().chars().last().unwrap()
    } else if flag == "last" {
        word.trim().chars().next().unwrap()
    } else {
        panic!("unknown flag")
    }
}
fn main() {
    let mut word1: String = String::new();
    let mut word2: String = String::new();

    write(&mut word1);
    write(&mut word2);

    println!("{}", chr(&word1, "first") == chr(&word2, "last"));

}
