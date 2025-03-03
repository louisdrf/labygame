use std::time::Instant;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Secrets {
    pub secrets: Arc<Mutex<HashMap<u128, Instant>>>,
}

impl Secrets {
    pub fn new() -> Self {
        Secrets {
            secrets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn sum_modulo(&self, modulo: u64) -> u64 {
        let sum: u128 = self.secrets.lock().unwrap().keys().copied().sum();
        (sum % modulo as u128) as u64
    }

    pub fn update_secret(&self, secret: u128) {
        let mut secrets = self.secrets.lock().unwrap();
        secrets.insert(secret, Instant::now());
    }
}
