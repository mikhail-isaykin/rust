fn main() {
    let mut sum: u32 = 0;

    for num in 1..=100 {
        sum += num;
    }
    println!("{}", sum);
}