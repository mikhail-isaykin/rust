fn main() {
    let txt: &str = "123456789";

    let mut sum: u32 = 0;

    for chr in txt.chars() {
        sum += chr.to_digit(10).unwrap();

    }
    println!("{}", sum);
}