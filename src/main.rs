fn main() {
    let mut current: f64 = 7435421243.0;

    let units: [&str; 4] = ["byte", "kb", "mb", "gb"];

    println!("{:.3}: {}", current, units[0]);

    for i in 1..=3 {
        current /= 1024.0;
        println!("{:.3}: {}", current, units[i]);
    }
}