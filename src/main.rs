fn main() {
    let mut sum_sq_odd: i32 = 0;

    for num in (1..=100i32).step_by(2) {
        sum_sq_odd += num.pow(2);
    }
    println!("{}", sum_sq_odd);
}