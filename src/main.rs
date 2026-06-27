fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let first = &strs[0];
    for (i, ch) in first.char_indices() {
        for s in &strs[1..] {
            if s.as_bytes().get(i) != Some(&(ch as u8)) {
                return first[..i].to_string();
            }
        }
    }
    first.clone()
}

fn main() {
    let strs = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    println!("{}", longest_common_prefix(strs)); // "fl"
}
