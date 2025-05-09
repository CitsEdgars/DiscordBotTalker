use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const BANK_LOCATION: &str = "bank.json";


#[derive(Serialize, Deserialize, Debug)]
pub struct LocalBank {
    pub balances: HashMap<String, u32>,
}

impl LocalBank {
    pub fn load_bank() -> Self {
        let mut file = File::open(BANK_LOCATION).unwrap_or_else(|_| {
            println!("Creating new storage at {}", BANK_LOCATION);
            File::create(BANK_LOCATION).expect("Failed to create file")
        });

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap_or(0);

        if contents.trim().is_empty() {
            LocalBank {
                balances: HashMap::new(),
            }
        } else {
            serde_json::from_str(&contents).unwrap_or_else(|err| {
                eprintln!("Failed to parse balances: {}. Starting fresh.", err);
                LocalBank {
                    balances: HashMap::new(),
                }
            })
        }
    }

    pub fn save_balance(&self) {
        let json = serde_json::to_string_pretty(&self).expect("Failed to serialize");
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(BANK_LOCATION)
            .expect("Failed to open file for writing");
        file.write_all(json.as_bytes()).expect("Failed to write data");
    }

    pub fn add_tokens(&mut self, user: &str, amount: u32) {
        let balance = self.balances.entry(user.to_string()).or_insert(0);
        *balance += amount;
        println!("{} earned {} tokens.", user, amount);
    }

    pub fn remove_tokens(&mut self, user: &str, amount: u32) -> bool {
        if let Some(balance) = self.balances.get_mut(user) {
            if *balance >= amount {
                *balance -= amount;
                println!("{} spent {} tokens.", user, amount);
                return true;
            } else {
                println!("{} does not have enough tokens.", user);
            }
        } else {
            println!("{} not found in the system.", user);
        }
        false
    }

    pub fn get_balance(&self, user: &str) -> u32 {
        *self.balances.get(user).unwrap_or(&0)
    }
}
