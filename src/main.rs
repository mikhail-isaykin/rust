fn main() {
    let arr: [u8; 6] = [1, 2, 3, 0, 4, 5];

    for num in arr.iter() {
        if *num == 0 {
            break;
        }
        print!("{} ", num);
    }
}
