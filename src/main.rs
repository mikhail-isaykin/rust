fn main() {
    let nums = Box::new([3, 7, 2, 9, 5]);

    let mut mx = nums[0];
    for &x in nums.iter() {
        if x > mx {
            mx = x;
        }
    }

    println!("{mx}");
}