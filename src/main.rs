struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    fn greet(&self) -> String {
        format!("Hi, my name is {} and I'm {} years old", self.name, self.age)
    }

    fn have_birthday(&mut self) {
        self.age += 1;
    }
}

fn main() {
    let mut user = User::new("Alex", 21);
    println!("{}", user.greet());
    user.have_birthday();
}
