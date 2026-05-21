use std::mem;

fn main() {
    let arr: [i8; 5] = [1, 2, 3, 4, 5];

    println!("{}", mem::size_of_val(&arr));
}