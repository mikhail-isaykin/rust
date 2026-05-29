use std::collections::HashSet;

fn find_pairs(nums: &[i32], target: i32) -> Vec<(i32, i32)> {
    let mut seen: HashSet<i32> = HashSet::new();
    let mut result: Vec<(i32, i32)> = Vec::new();

    for &n in nums.iter() {
        let complement = target - n;
        if seen.contains(&complement) {
            let pair = (complement.min(n), complement.max(n));
            if !result.contains(&pair) {
                result.push(pair);
            }
        }
        seen.insert(n);
    }

    result
}

fn main() {
    let nums = vec![1, 5, 3, 7, 2, 8, 4, 6];
    let target = 9;

    let pairs = find_pairs(&nums, target);
    println!("Пары с суммой {}:", target);
    for (a, b) in &pairs {
        println!("  {} + {} = {}", a, b, target);
    }
}
