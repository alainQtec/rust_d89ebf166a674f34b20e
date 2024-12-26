fn struct_demo() {
    println!("++++++++ STRUCT demo ++++++++");
    let mut account: BankAccount = BankAccount {
        owner: String::from("alain herve"),
        balance: 150.0,
    };
    println!("Profile      : {}", account.get_profile());
    account.withdraw(50.0);
    account.deposit(20.0);
    println!("Transactions : [-50, + 20]\n");
    println!("Balance      : {}", account.check_balance());
    println!("Profile      : {}", account.get_profile());
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }
    fn check_balance(&self) -> f64 {
        return self.balance;
    }
    fn get_profile(&self) -> String {
        return format!("({}, {})", self.owner, self.check_balance());
    }
}
