fn main() {
    let arr: [u8; 5] = [1, 2, 3, 4, 5];

    let sum: u16 = arr.iter().map(|&num| (num as u16).pow(2)).sum();

    print!("{}", sum);
}
