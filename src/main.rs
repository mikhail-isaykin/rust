fn mean_of_evens(nums: &[i32]) -> Option<f64> {
    let evens: Vec<i32> = nums.iter().filter(|&&x| x % 2 == 0).copied().collect();
    if evens.is_empty() {
        return None;
    }
    let sum: i32 = evens.iter().sum();
    Some(sum as f64 / evens.len() as f64)
}

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    match mean_of_evens(&v) {
        Some(avg) => println!("Среднее чётных: {}", avg), // 4.0
        None => println!("Чётных нет"),
    }
}
