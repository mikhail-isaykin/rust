fn main() {
    let result: f32 = (1..=100).map(|num| num as f32).sum::<f32>() / (1..=100).count() as f32;

    println!("{}", result)
}