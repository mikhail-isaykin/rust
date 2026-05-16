fn main() {
    let num1: u16 = 12;
    let num2: u16 = 13;

    let first1: u16 = num1.to_string().chars().next().unwrap().to_digit(10).unwrap() as u16;
    let first2: u16 = num2.to_string().chars().next().unwrap().to_digit(10).unwrap() as u16;

    println!("{}", first1 == first2);
}
