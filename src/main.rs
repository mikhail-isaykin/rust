fn max_subarray(nums: Vec<i32>) -> i32 {
    let mut best = nums[0];
    let mut cur = nums[0];
    for &n in &nums[1..] {
        cur = n.max(cur + n);
        best = best.max(cur);
    }
    best
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("{}", max_subarray(nums)); // 6
}
