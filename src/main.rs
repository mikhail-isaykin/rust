fn main() {
    let mb: f32 = 35.5;
    let klb: f32 = mb * 1024.0;
    let byte: f32 = klb * 1024.0 as f32;

    println!("{}", byte);
}