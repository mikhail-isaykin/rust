fn main() {
    let nums = Box::new(vec![5, 12, 8, 15, 20]);

    let mut count = 0;
    for &x in nums.iter() {
        if x > 10 {
            count += 1;
        }
    }

    println!("{count}");
}