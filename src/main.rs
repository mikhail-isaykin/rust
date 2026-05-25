fn main() {
    let arr: [&str; 3] = ["2025", "12", "31"];

    print!("{}", arr.into_iter().rev().collect::<Vec<_>>().join("-"));
}
