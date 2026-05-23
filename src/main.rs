fn main() {
    let arr: [u8; 5] = [1, 2, 3, 4, 5];

    let sum_sqrt: f32 = arr.iter().map(|num| (*num as f32).sqrt()).sum();

    println!("{}", sum_sqrt);
}
