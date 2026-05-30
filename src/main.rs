fn reverse_strings(words: &[&str]) -> Vec<String> {
    words.iter().map(|w| w.chars().rev().collect()).collect()
}

fn main() {
    let v = vec!["hello", "rust", "", "world"];
    let reversed = reverse_strings(&v);
    println!("{:?}", reversed); // ["olleh", "tsur", "", "dlrow"]
}
