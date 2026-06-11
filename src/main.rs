use std::collections::HashMap;

fn first_unique(nums: Vec<i32>) -> Option<i32> {
    let mut counts: HashMap<i32, usize> = HashMap::new();

    // Считаем частоты
    for num in &nums {
        *counts.entry(*num).or_insert(0) += 1;
    }

    // Ищем первое уникальное число
    for num in nums {
        if counts.get(&num) == Some(&1) {
            return Some(num);
        }
    }

    None
}

fn main() {
    let nums = vec![4, 2, 1, 2, 1];

    println!("{:?}", first_unique(nums)); // Some(4)
}