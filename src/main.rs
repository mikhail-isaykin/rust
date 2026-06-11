use std::rc::Rc;

fn main() {
    let message = Rc::new(String::from("Hello, Rust!"));

    let a = Rc::clone(&message);
    let b = Rc::clone(&message);

    println!("{}", a);
    println!("{}", b);

    println!("Owners: {}", Rc::strong_count(&message));
}