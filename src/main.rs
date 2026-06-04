fn fizzbuzz(n: u32) -> (u32, u32, u32) {
    let (mut fizz, mut buzz, mut fizzbuzz) = (0, 0, 0);

    for i in 1..=n {
        match (i % 3, i % 5) {
            (0, 0) => fizzbuzz += 1,
            (0, _) => fizz += 1,
            (_, 0) => buzz += 1,
            _      => {}
        }
    }

    (fizz, buzz, fizzbuzz)
}

fn main() {
    let n = 15;
    let (fizz, buzz, fizzbuzz) = fizzbuzz(n);
    println!("n={} → Fizz:{}, Buzz:{}, FizzBuzz:{}", n, fizz, buzz, fizzbuzz);
}
