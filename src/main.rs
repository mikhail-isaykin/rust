use std::collections::HashSet;

fn main() {
    let nums = vec![4, 1, 2, 1, 3, 4, 2, 5, 3];

    let mut seen = HashSet::new();
    let unique: Vec<i32> = nums
        .iter()
        .filter(|&&n| seen.insert(n))
        .copied()
        .collect();

    println!("{:?}", unique);
}
