use std::collections::HashMap;

fn first_duplicate(nums: &[i32]) -> Option<(usize, usize)> {
    let mut seen: HashMap<i32, usize> = HashMap::new();

    for (i, &n) in nums.iter().enumerate() {
        if let Some(&j) = seen.get(&n) {
            return Some((j, i));
        }
        seen.insert(n, i);
    }

    None
}

fn main() {
    let nums = [3, 8, 5, 1, 8, 4, 5];

    match first_duplicate(&nums) {
        Some((i, j)) => println!("Дубликат на индексах: {}, {}", i, j),
        None => println!("Дубликатов нет"),
    }
}
