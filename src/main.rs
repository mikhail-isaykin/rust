fn main() {
    let num: u16 = 12;

    for n in 1..=num {
        if num % n == 0 {
            println!("{}", n);
        }
    }
}
