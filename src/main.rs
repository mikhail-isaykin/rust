use std::collections::HashMap;

fn group_anagrams(words: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for word in words {
        let mut key: Vec<char> = word.chars().collect();
        key.sort_unstable();
        let key: String = key.into_iter().collect();
        map.entry(key).or_default().push(word);
    }
    map.into_values().collect()
}

fn main() {
    let words = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    let groups = group_anagrams(words);
    println!("{:?}", groups);
}
