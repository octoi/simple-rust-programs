// Basic login system

struct User {
    username: String,
    password: String,
    message: String,
}

impl User {
    fn new( username: &str, password: &str, message: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            message: message.to_string(),
        }
    }
}

fn main() {
    let users: [User; 3] = [
        User::new("rust", "ferris123", "Hello rustacean"),
        User::new("go", "gagoo", "Go gang!"),
        User::new("javascript", "nan!=Nan=true", "Js is booootiful !"),
    ];
}