fn main() {
    let arr: [u8; 5] = [1, 2, 3, 4, 5];

    let mut txt: String = String::new();

    for num in arr {
        txt.push(char::from_digit(num as u32, 10).unwrap());
    }
    print!("{}", txt);
}
