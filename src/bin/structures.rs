fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("someone1@example.com");

    let user2 = build_user(
        String::from("someone@example.com"),
        String::from("someone2"),
    );

    let user3 = User {
        email: String::from("someone3@example.com"),
        username: String::from("someone3"),
        ..user2
    };
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
