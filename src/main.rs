fn main() {
    let arr: [i8; 5] = [1, 2, -3, 4, -5];

    let sum: u8 = arr.iter().filter(|&&num| num > 0).map(|&num| num as u8).sum();

    println!("{}", sum);
}
