fn split_even_odd(nums: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut evens: Vec<i32> = Vec::new();
    let mut odds: Vec<i32> = Vec::new();

    for &n in nums.iter() {
        if n % 2 == 0 {
            evens.push(n);
        } else {
            odds.push(n);
        }
    }

    (evens, odds)
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let (evens, odds) = split_even_odd(&nums);
    println!("Чётные: {:?}", evens);
    println!("Нечётные: {:?}", odds);
}
