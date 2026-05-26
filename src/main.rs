use std::collections::HashMap;


fn are_anagrams(a: &str, b: &str) -> bool {
    if a.chars().count() != b.chars().count() {
        return false;
    }

    let mut counts: HashMap<char, i32> = HashMap::new();
    for c in a.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    for c in b.chars() {
        *counts.entry(c).or_insert(0) -= 1;
    }

    counts.values().all(|&v| v == 0)
}

fn main() {
    let pairs = [("listen", "silent"), ("hello", "world"), ("rust", "trus")];

    for (a, b) in pairs {
        println!("{} / {} -> {}", a, b, are_anagrams(a, b));
    }
}
