use chrono::prelude::*;
use pwhash::bcrypt;
use sanitize_html::{rules::predefined::DEFAULT, sanitize_str};
use std::fs::File;
use std::io::prelude::*;

pub fn now() -> chrono::naive::NaiveDateTime {
    Utc::now().naive_local()
}

pub fn read_file_to_string(path: &String) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn read_file_to_bytes(path: &String) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut contents = Vec::<u8>::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

pub fn encrypt(password: &str) -> String {
    bcrypt::hash(password).unwrap()
}

pub fn verify(password: &str, hashed: &str) -> bool {
    bcrypt::verify(password, hashed)
}

pub fn sanitize_html(input: &str) -> String {
    sanitize_str(&DEFAULT, input).unwrap()
}

#[test]
fn test_encryption() {
    // Hash a password with default parameters.
    let h_new = encrypt("password");

    assert!(verify("password", &h_new));
}
