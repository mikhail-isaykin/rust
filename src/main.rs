use std::collections::HashMap;


fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
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
    let nums = [2, 7, 11, 15];

    match two_sum(&nums, 9) {
        Some((i, j)) => println!("Индексы: {}, {}", i, j),
        None => println!("Пара не найдена"),
    }
}
