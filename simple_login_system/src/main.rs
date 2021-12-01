// Basic login system

use std::io;
use std::io::Write;
use std::process;

struct User {
    username: String,
    password: String,
    message: String,
}

impl User {
    fn new(username: &str, password: &str, message: &str) -> Self {
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

    let username = input("Enter your username: ");
    let password = input("Enter your password: ");

    // login
    for user in users {
        login(&user, &username.trim(), &password.trim());
    }

    println!("Seems like there is no user with username {}", username);
}

fn login(user: &User, username: &str, password: &str) {
    if user.username == username.to_string() {
        if user.password == password.to_string() {
            println!("Welcome back {} 🥳", username);
            println!("secret message: {}", user.message);
        } else {
            println!("Incorrect password for {}", username);
        }

        process::exit(0);
    }
}


fn input(message: &str) -> String {
    let mut input_value = String::new();

    print!("{}", message);

    io::stdout().flush().expect("Failed to take input");
    io::stdin().read_line(&mut input_value).expect("Failed to read line");

    return input_value;
}