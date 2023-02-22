use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rpassword::read_password_from_tty;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

fn main() {
    let mut passwords = load_passwords().unwrap_or_else(HashMap::new);

    let command = std::env::args().nth(1);
    match command.as_deref() {
        Some("add") => add_password(&mut passwords),
        Some("get") => get_password(&passwords),
        Some("list") => list_passwords(&passwords),
        Some("generate") => generate_password(),
        _ => println!("Usage: password_manager <add|get|list|generate>"),
    }

    if let Err(e) = save_passwords(&passwords) {
        println!("Failed to save passwords: {}", e);
    }
}

fn load_passwords() -> std::io::Result<HashMap<String, String>> {
    let path = get_password_file_path()?;
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let passwords = serde_json::from_reader(reader)?;
    Ok(passwords)
}

fn save_passwords(passwords: &HashMap<String, String>) -> std::io::Result<()> {
    let path = get_password_file_path()?;
    let file = OpenOptions::new().write(true).truncate(true).open(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, passwords)?;
    Ok(())
}

fn add_password(passwords: &mut HashMap<String, String>) {
    println!("Enter service name:");
    let service = read_line();
    println!("Enter password:");
    let password = read_password_from_tty(None).unwrap();
    passwords.insert(service, password);
    println!("Password added for service: {}", service);
}

fn get_password(passwords: &HashMap<String, String>) {
    println!("Enter service name:");
    let service = read_line();
    match passwords.get(&service) {
        Some(password) => println!("Password for service {}: {}", service, password),
        None => println!("No password found for service: {}", service),
    }
}

fn list_passwords(passwords: &HashMap<String, String>) {
    if passwords.is_empty() {
        println!("No passwords stored");
    } else {
        println!("Stored passwords:");
        for (service, _) in passwords {
            println!("{}", service);
        }
    }
}

fn generate_password() {
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    println!("Generated password: {}", password);
}

fn get_password_file_path() -> std::io::Result<PathBuf> {
    let mut path = dirs::home_dir().unwrap_or_default();
    path.push(".passwords.json");
    Ok(path)
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
