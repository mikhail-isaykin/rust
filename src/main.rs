fn main() {
    let gb: f32 = 35.24;
    let mb: f32 = gb * 1024.0;
    let kb: f32 = mb * 1024.0;
    let byte: f32 = kb * 1024.0;

    println!("{:?}", [gb, mb, kb, byte])
}