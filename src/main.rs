fn main() {
    let mut num: f32 = 1000.0;
    let mut counter: u32 = 0;

    while num >= 10.0 {
        num /= 2.0;
        counter += 1;
    }

    println!("{}", counter);
}