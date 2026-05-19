fn main() {
    let num: u32 = 1234567;
    let txt: String = num.to_string();

    println!("{:?}", txt.chars().filter_map(|s| s.to_digit(10)).sum::<u32>());
}