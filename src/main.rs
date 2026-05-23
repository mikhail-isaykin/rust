fn main() {
    let arr: [u8; 4] = [1, 2, 3, 4];

    let arr: [u8; 4] = arr.map(|num| num * 2);

    println!("{:?}", arr);
}
