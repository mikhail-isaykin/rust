fn rotate_right(nums: &[i32], k: usize) -> Vec<i32> {
    if nums.is_empty() {
        return Vec::new();
    }

    let n = nums.len();
    let shift = k % n;
    let mut result = Vec::with_capacity(n);

    result.extend_from_slice(&nums[n - shift..]);
    result.extend_from_slice(&nums[..n - shift]);

    result
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;

    let rotated = rotate_right(&nums, k);
    println!("Исходный: {:?}", nums);
    println!("После поворота на {}: {:?}", k, rotated);
}
