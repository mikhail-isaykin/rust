fn main() {
    let num: u8 = 22;
    let txt: String = num.to_string();

    let first: u32 = txt.chars().next().unwrap().to_digit(10).unwrap();
    let last: u32 = txt.chars().last().unwrap().to_digit(10).unwrap();

    println!("{}", last > first);
}
