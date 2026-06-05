fn main() {
    let nums = vec![2, 7, 11, 15, 3, 6];
    let target = 9;

    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                println!("[{}, {}]", i, j);
            }
        }
    }
}
