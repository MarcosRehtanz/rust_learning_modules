use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct User {
    username: String,
    hash_password: u64,
}

impl User {
    pub fn new(username: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            hash_password: hash_password(password),
        }
    }
    pub fn get_username(&self) -> &String {
        &self.username
    }
    pub fn set_password(&mut self, new_password: &str) {
        self.hash_password = hash_password(new_password);
    }
    pub fn login(&self, username: &str, password: &str) -> bool {
        if self.username != username {
            return false;
        }
        if self.hash_password != hash_password(password) {
            return false;
        }
        true
    }
}

fn hash_password(password: &str) -> u64 {
    let mut hashed = DefaultHasher::new();
    password.hash(&mut hashed);
    hashed.finish()
}
