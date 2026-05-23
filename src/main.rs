fn main() {
    let num: u16 = 12345;

    let nums: Vec<u16> = num
        .to_string()
        .chars()
        .map(|chr| chr.to_digit(10).unwrap() as u16)
        .collect();

    for num in &nums[(nums.len() - 3)..nums.len()] {
        println!("{}", num);
    }
}
