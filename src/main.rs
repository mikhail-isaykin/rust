fn second_max(nums: &[i32]) -> Option<i32> {
    let mut max = i32::MIN;
    let mut second = i32::MIN;

    for &n in nums.iter() {
        if n > max {
            second = max;
            max = n;
        } else if n > second && n != max {
            second = n;
        }
    }

    if second == i32::MIN { None } else { Some(second) }
}

fn main() {
    let nums = vec![3, 1, 9, 7, 9, 2, 5];

    match second_max(&nums) {
        Some(n) => println!("Второй максимум: {}", n),
        None => println!("Второго максимума нет"),
    }
}
