fn is_palindrome(s: &str) -> bool {
    let cleaned: Vec<char> = s
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();

    let reversed: Vec<char> = cleaned.iter().rev().cloned().collect();
    cleaned == reversed
}

fn main() {
    let tests = vec![
        ("racecar",      true),
        ("A man a plan", true),
        ("hello",        false),
        ("",             true),
        ("Aba",          true),
    ];

    for (input, expected) in tests {
        let result = is_palindrome(input);
        let status = if result == expected { "✓" } else { "✗" };
        println!("{} {:20} → {}", status, input, result);
    }
}
