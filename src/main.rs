use std::collections::HashMap;

fn main() {
    let text = "apple banana apple orange banana apple";

    let mut frequencies: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        *frequencies.entry(word).or_insert(0) += 1;
    }

    for (word, count) in &frequencies {
        println!("{}: {}", word, count);
    }
}