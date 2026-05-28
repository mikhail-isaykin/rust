fn max_subarray_sum(nums: &[i32], k: usize) -> Option<i32> {
    if k == 0 || nums.len() < k {
        return None;
    }

    let mut window_sum: i32 = nums[..k].iter().sum();
    let mut max_sum = window_sum;

    for i in k..nums.len() {
        window_sum += nums[i] - nums[i - k];
        if window_sum > max_sum {
            max_sum = window_sum;
        }
    }

    Some(max_sum)
}

fn main() {
    let nums = vec![2, 1, 5, 1, 3, 2, 7, 1];
    let k = 3;

    match max_subarray_sum(&nums, k) {
        Some(s) => println!("Максимальная сумма окна длины {}: {}", k, s),
        None => println!("Некорректные данные"),
    }
}
