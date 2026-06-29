fn filter_even_numbers(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        .collect()
}

fn main() {
    let values = vec![3, 8, 11, 14, 21, 26, 33, 40];

    let even_values = filter_even_numbers(values);

    println!("Filtered values:");
    
    for value in even_values {
        println!("{}", value);
    }
}
