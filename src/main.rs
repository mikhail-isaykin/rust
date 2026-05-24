fn main() {
    let arr: [&str; 3] = ["123", "456", "789"];

    let nums: [u16; 3]  = arr.map(|s| s.parse().unwrap());

    print!("{:?}", nums);
}
