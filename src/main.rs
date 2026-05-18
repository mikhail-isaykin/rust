fn main() {
    let txt1: &str = "123";
    let txt2: &str = "456";
    let txt3: &str = "789";

    let mut result: String = txt1.to_string();
    result.push_str(txt2);
    result.push_str(txt3);

    println!("{}", result);
}