fn main() {
    let num1: u16 = 36;
    let num2: u16 = 24;
    let num3: u16 = 12;

    let m: u16 = [num1, num2, num3].into_iter().max().unwrap();

    println!("{}", m);
}