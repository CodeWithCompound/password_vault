use argon2::{self, Config};
use rand::Rng;
use std::io::{self, Write};
// in the following function we read user input and make sure it's a string
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf.trim().to_string()
}

fn main() {
    let password = read_input("Enter a password: ");

    // salt + hash
    let salt: [u8; 16] = rand::thread_rng().gen();
    let config = Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config)
        .expect("Failed to hash");

    println!("\nStored hash:\n{}\n", hash);

    // verify with up to 3 attempts
    let mut attempts = 0;
    let max_attempts = 3;
    while attempts < max_attempts {
        println!("Now please log in below with the password you set earlier.");
        let verify_pass = read_input("Enter password again to verify: ");
        let ok = argon2::verify_encoded(&hash, verify_pass.as_bytes()).unwrap_or(false);
        // check password
        if ok {
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
