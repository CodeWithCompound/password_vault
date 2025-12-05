use argon2::{self, Config};
use rand::Rng;
use std::io::{self, Write};

fn main() {
    print!("Enter password: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let password = input.trim();


    let salt: [u8; 16] = rand::thread_rng().gen();

    let config = Config::default();
    let hash =
        argon2::hash_encoded(password.as_bytes(), &salt, &config).expect("Failed to hash");

    println!("\nStored hash:");
    println!("{}", hash);

    print!("\nEnter password again to verify: ");
    io::stdout().flush().unwrap();

    let mut verify_input = String::new();
    io::stdin().read_line(&mut verify_input).unwrap();
    let verify_pass = verify_input.trim();

    // Check match
    let ok = argon2::verify_encoded(&hash, verify_pass.as_bytes()).unwrap();

    println!("\nMatch? {}\n", ok);
}
