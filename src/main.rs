fn main() {
    let num: u8 = 4;

    match num {
        1..=3 => println!("Winter"),
        4..=6 => println!("Spring"),
        7..=9 => println!("Summer"),
        10..=12 => println!("Autumn"),
        _ => println!("{}", { false }),
    }
}
