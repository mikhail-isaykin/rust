fn main() {
    let mut sum: u32 = 0;

    for num in 1u32..=100 {
        sum += num.pow(2);
    }
    
    println!("{}", sum);
}
