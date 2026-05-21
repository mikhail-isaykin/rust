fn main() {
    let num: u16 = 12345;

    let txt: String = num.to_string();

    let first: u32 = txt.chars().next().unwrap().to_digit(10).unwrap();
    let last: u32 = txt.chars().last().unwrap().to_digit(10).unwrap();

    println!("{}", first + last)
}