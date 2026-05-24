fn main() {
    let arr: [i8; 6] = [-1, 2, -3, 4, 5, 11];

    let sum: u8 = arr.iter().filter_map(|&num| (num > 0 && num < 10).then_some(num as u8)).sum();

    print!("{}", sum);
}
