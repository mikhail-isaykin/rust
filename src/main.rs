struct User {
    username: String,
    active: bool,
    login_count: u32,
}

impl User {
    fn login(&mut self) {
        self.login_count += 1;
        self.active = true;
    }
}

fn main() {
    let mut user = User {
        username: String::from("dev_user"),
        active: false,
        login_count: 0,
    };

    user.login();

    println!(
        "{} | active: {} | logins: {}",
        user.username,
        user.active,
        user.login_count
    );
}
