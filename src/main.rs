fn main() {
    let num: u16 = 12345;

    let sum: u32 = num
        .to_string()
        .chars()
        .filter_map(|chr| chr.to_digit(10))
        .filter(|dgt| dgt % 2 == 0)
        .sum();

    println!("{}", sum)
}