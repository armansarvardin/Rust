#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account)
    }
}

impl Account {
    fn new(id: u32, balance: i32, holder: String) -> Self {
        Account {
            id,
            balance,
            holder,
        }
    }

    fn deposit(&mut self, amount: i32) {}

    fn withdraw(&mut self, amount: i32) {}
}

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, 100, "John Doe".to_string());
    bank.add_account(account);
    println!("{:#?}", bank);
}
