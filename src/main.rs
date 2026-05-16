use std::io;

fn main() {
    let mut text: String = String::new();

    io::stdin()
        .read_line(&mut text)
        .unwrap();

    let len_text: usize = text.trim().chars().count();

    if len_text > 1 {
        println!("{}", text.trim().chars().nth(len_text - 2).unwrap());
    } else {
        println!("{}", { false });
    }
}
