use std::io;

fn main() {
    let mut range: String = String::new();

    io::stdin()
        .read_line(&mut range)
        .unwrap();
    
    let range: u32 = range.trim().parse().unwrap();

    for exp in 0..range {
        println!("{}", 2u32.pow(exp));
    }
}