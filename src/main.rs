fn main() {
    let num: i32 = 123;
    let text: String = num.to_string();

    println!("{}", &text.chars().last().unwrap());
}
