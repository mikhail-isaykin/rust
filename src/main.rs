fn main() {
    let arr: [u32; 5] = [1, 2, 3, 4, 5];

    println!("{}", arr.iter().sum::<u32>());
}