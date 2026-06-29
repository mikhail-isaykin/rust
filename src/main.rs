use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<&str, i32> = HashMap::new();

    scores.insert("Alice", 42);
    scores.insert("Bob", 17);
    scores.insert("Charlie", 29);

    for (name, score) in &scores {
        println!("{} => {}", name, score);
    }

    let average: i32 = scores.values().sum::<i32>() / scores.len() as i32;

    if average > 25 {
        println!("Average score is high: {}", average);
    } else {
        println!("Average score is low: {}", average);
    }
}
