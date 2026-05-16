fn main() {
    let num1: u16 = 142;
    let num2: u16 = 142;

    if num1 > num2 {
        println!("{}", num1);
    } else if num2 > num1 {
        println!("{}", num2);
    } else {
        println!("Числа равны");
    }
}