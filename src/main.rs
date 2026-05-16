fn main() {
    let num: u8 = 30;
    
    match num {
        1..=15 => println!("{}", 1),
        16..=30 => println!("{}", 2),
        31..=45 => println!("{}", 3),
        46..=60 => println!("{}", 4),
        _ => println!("{}", { false }),

    }
}
