use argon2::{self, Config};
use rand::Rng;
use std::{fs::read, io::{self, Write}};
// in the following function we read user input and make sure it's a string
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf.trim().to_string()
}

fn check_user(user: &String, verify_user: &String ) -> bool {
if user == verify_user {
    println!("Username match!");
    true
} else {
    println!("Username does not match. Exiting.");
    false
    }
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
    let hash_pw = argon2::hash_encoded(password.as_bytes(), &salt, &config)
        .expect("Failed to hash_pw");

    println!("\nStored hash_pw:\n{}\n", hash_pw);

    // verify with up to 3 attempts
    let mut attempts = 0;
    let max_attempts = 3;
    while attempts < max_attempts {
        println!("--- Login ---");
        println!("Now please log in below with the username & password you set earlier.");
        let verify_user = read_input("Enter username again to verify:");
         if !check_user(&user, &verify_user) {
            // change this later to allow re-entry instead of exit
            break;
         } 
        let verify_pass = read_input("Enter password again to verify: ");
        //let ok = argon2::verify_encoded(&hash_pw, verify_pass.as_bytes()).unwrap_or(false) && ;
                // check password
       if check_password(&hash_pw, &verify_pass) {
              println!("\n Password match! You're in.");
            break;
         } else {
            attempts += 1;
            let left = max_attempts - attempts;
            println!("\n Password does not match. Attempts left: {}\n", left);
         }
    }
    if attempts == max_attempts {
        println!("Too many failed attempts. Please try again later.");
    }
}
