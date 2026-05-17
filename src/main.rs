use std::collections::HashMap;
use std::io;

fn main() {

    let mut num_trophies_input = String::new();
    io::stdin()
        .read_line(&mut num_trophies_input)
        .expect("Не удалось прочитать количество трофеев");

    let num_trophies: u32 = num_trophies_input
        .trim()
        .parse()
        .expect("Пожалуйста, введите корректное число");

    let mut trophies: HashMap<String, u32> = HashMap::new();

    for _ in 0..num_trophies {
        let mut trophy: String = String::new();

        io::stdin()
            .read_line(&mut trophy)
            .unwrap();

        trophies.entry(trophy.trim().to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for (key, value) in &trophies {
        println!("{}: {}", key, value);
    }

}