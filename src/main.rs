fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    (0..n / 2).all(|i| chars[i] == chars[n - 1 - i])
}

fn main() {
    let words = ["radar", "rust", "level", "hello"];

    for w in words {
        println!("{} -> {}", w, is_palindrome(w));
    }
}