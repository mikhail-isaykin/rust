fn main() {
    let mut arr: [u8; 10] = [0; 10];

    for i in 0..=9 {
        arr[i] = i as u8 + 1;
    }

    println!("{:?}", arr);
}