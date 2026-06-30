struct BankAccount {
    owner: String,
    balance: i32,
}

impl BankAccount {
    fn new(owner: &str, balance: i32) -> Self {
        Self {
            owner: owner.to_string(),
            balance,
        }
    }

    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    fn get_balance(&self) -> i32 {
        self.balance
    }
}

fn main() {
    let mut acc = BankAccount::new("Ivan", 100);
    acc.deposit(50);
    println!("Balance: {}", acc.get_balance());
}
