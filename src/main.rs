use std::collections::HashSet;

fn dedup_keep_order(nums: &[i32]) -> Vec<i32> {
    let mut seen: HashSet<i32> = HashSet::new();
    let mut result: Vec<i32> = Vec::new();

    for &n in nums.iter() {
        if seen.insert(n) {
            result.push(n);
        }
    }

    result
}

fn main() {
    let nums = vec![1, 3, 2, 3, 5, 1, 4, 2];

    let unique = dedup_keep_order(&nums);
    println!("Исходный: {:?}", nums);
    println!("Без дубликатов: {:?}", unique);
}
