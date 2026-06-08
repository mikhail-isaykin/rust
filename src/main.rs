use std::rc::Rc;

fn main() {
    let nums = Rc::new(vec![1, 2, 3, 4, 5, 6]);

    let mut sum = 0;
    for &x in nums.iter() {
        if x % 2 == 0 {
            sum += x;
        }
    }

    println!("{sum}");
}