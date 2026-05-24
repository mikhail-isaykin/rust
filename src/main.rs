fn main() {
    let arr: [u8; 5] = [1, 2, 3, 4, 5];

    for num in arr.iter().rev() {
        print!("{} ", num);
    }
}
