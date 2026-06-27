fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    use std::collections::HashMap;
    let mut seen: HashMap<i32, usize> = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        if let Some(&j) = seen.get(&(target - n)) {
            return Some((j, i));
        }
        seen.insert(n, i);
    }
    None
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    println!("{:?}", two_sum(nums, 9)); // Some((0, 1))
}
