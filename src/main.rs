fn main() {
    let arr: [u32; 5] = [1, 2, 3, 4, 5];

    let sum: u32 = arr.map(|n: u32| n.pow(2)).iter().sum();

    println!("{}", sum);
}