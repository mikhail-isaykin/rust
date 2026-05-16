fn main() {
    let num: i32 = 323;

    let text: String = num.to_string();

    let first: char = text.chars().next().unwrap();
    let last: char = text.chars().last().unwrap();

    println!("{}", first == last);
}