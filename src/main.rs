fn main() {
    let arr: [u32; 6] = [1, 2, 3, 4, 5, 8];

    let avg: f32 = arr.map(|n: u32| n as f32).iter().sum::<f32>() / arr.len() as f32;

    println!("{}", avg);
}