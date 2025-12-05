use argon2::{self, Config};
use rand::Rng;
use std::io::{self, Write};
// in the following function we read user input and make sure it's a string
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
    buf.trim().to_string()
}

fn check_user(user: &String, verify_user: &String, attempts: u32, max_attempts: u32) -> bool {
    if user == verify_user {
        true
    } else {
        wrong_attempt(attempts, max_attempts);
        false
    }
}
fn wrong_attempt(attempts: u32, max_attempts: u32) {
    if max_attempts == attempts {
        println!("No attempts left.");
    } else {
        println!(
            "Wrong username. Attempts left: {}",
            max_attempts - attempts - 1
        );
        return;
    };
}

fn check_password(hash_pw: &String, verify_pass: &String) -> bool {
    argon2::verify_encoded(&hash_pw, verify_pass.as_bytes()).unwrap_or(false)
}
fn main() {
    println!("Welcome to [app name here]! Please register now.");
    let user = read_input("Please enter a Username:");
    let password = read_input("Enter a password: ");

    // salt + hash_pw
    let salt: [u8; 16] = rand::thread_rng().gen();
    let config = Config::default();
    let hash_pw =
        argon2::hash_encoded(password.as_bytes(), &salt, &config).expect("Failed to hash_pw");

    println!("\nStored hash_pw:\n{}\n", hash_pw);

    // verify with up to 3 attempts
    let mut attempts = 0;
    let max_attempts = 3;
    while attempts < max_attempts {
        println!("--- Login ---");
        println!("Now please log in below with the username & password you set earlier.");
        let verify_user = read_input("Enter username again to verify:");
        if !check_user(&user, &verify_user, attempts, max_attempts) {
            attempts = attempts + 1;
            continue;
        }
        let verify_pass = read_input("Enter password again to verify: ");

        // check password
        if check_password(&hash_pw, &verify_pass) {
            println!("--- Login successful ---");
            println!("\n Successfully logged in! Welcome back, {}!\n", user);
            break;
        } else {
            attempts += 1;
            let left = max_attempts - attempts;
            println!("\n Password does not match. Attempts left: {}\n", left);
        }
    }
    if attempts == max_attempts {
        println!("--- Failed to login ---");
        println!("Too many failed attempts. Please try again later.");
    }
}
