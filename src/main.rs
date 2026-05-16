fn main() {
    let num: i32 = 123;
    let text: String = num.to_string();
    let first: i32 = text.chars().next().unwrap().to_digit(10).unwrap() as i32;
    let last: i32 = text.chars().last().unwrap().to_digit(10).unwrap() as i32;
    println!("{}", first + last);
}
