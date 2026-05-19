fn main() {
    let num: u32 = 1234567;
    
    let num_rev: u32 = num.to_string().chars().rev().collect::<String>().parse().unwrap();

    println!("{}", num_rev);
}
