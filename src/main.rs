use std::collections::HashMap;


fn first_unique_char(s: &str) -> Option<char> {
    // Считаем, сколько раз встречается каждый символ
    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    // Идём по строке заново и возвращаем первый с количеством 1
    s.chars().find(|c| counts[c] == 1)
}

fn main() {
    let tests = ["leetcode", "loveleetcode", "aabb", "swiss"];

    for t in tests {
        match first_unique_char(t) {
            Some(c) => println!("{:>13} -> '{}'", t, c),
            None => println!("{:>13} -> нет уникальных", t),
        }
    }
}
