const ACCOUNT_INACTIVE_PERIOD: &str = "10d";

fn main() {
    let user1 = User {
        username: String::from("isim"),
        email: String::from("isim@example.com"),
        status: AccountState::Active,
    };
    println!("user {}", user1.username);
    println!("\temail: {}", user1.email);

    let (account_state, _) = user1.is_active();
    println!("\taccount status: {:?}", account_state);

    // shadowed
    let mut user1 = user1;
    user1.set_inactive();

    println!(
        "deactivating user {} after {}...",
        user1.username, ACCOUNT_INACTIVE_PERIOD
    );
    let (account_state, _) = user1.is_active();
    println!("user {}", user1.username);
    println!("\taccount state: {:?}", account_state);
}

struct User {
    username: String,
    email: String,
    status: AccountState,
}

impl User {
    fn is_active(&self) -> (bool, Option<&Date>) {
        match &self.status {
            AccountState::Active => (true, None),
            AccountState::Inactive(inactive_date) => {
                println!("setting inactive date to {:?}...", inactive_date);
                (false, Some(inactive_date))
            }
        }
    }

    fn set_inactive(&mut self) {
        let inactive_date = Date {
            day: 27,
            month: 6,
            year: 2021,
        };
        self.status = AccountState::Inactive(inactive_date);
    }
}

enum AccountState {
    Active,
    Inactive(Date),
}

#[derive(Debug)]
struct Date {
    day: i8,
    month: i8,
    year: i16,
}
