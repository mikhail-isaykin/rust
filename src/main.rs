fn main() {
    let arr: [u8; 5] = [1, 2, 3, 4, 5];

    let evens: Vec<_> = arr.iter().filter(|&&num| num % 2 == 0).collect();

    print!("{:?}", evens);
}
