fn main() {
    let num: u16 = 12;

    match num {
        1..=10 => println!("{}", 1),
        11..=20 => println!("{}", 2),
        21..=31 => println!("{}", 3),
        _ => println!("invalid day"),
    }
}