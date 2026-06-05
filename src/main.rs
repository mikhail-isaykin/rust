fn main() {
    let mut nums = vec![5, 2, 8, 1, 9, 3, 7];
    let k = 3;

    nums.sort_unstable_by(|a, b| b.cmp(a));
    let top: Vec<i32> = nums.into_iter().take(k).collect();

    println!("{:?}", top);
}
