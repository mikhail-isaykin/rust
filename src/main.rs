fn main() {
    let mut arr: [u8; 10] = [0; 10];

    for (i, num) in (1..=10).rev().enumerate() {
        arr[i] = num as u8;
    }

    println!("{:?}", arr);
}
