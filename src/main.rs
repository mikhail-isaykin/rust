fn main() {
    let num: u8 = 12;

    let mut factorial: u32 = 1;

    for i in 1..=num {
        factorial *= i as u32;
    }
    println!("{}", factorial);
}