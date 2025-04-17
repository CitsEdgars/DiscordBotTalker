use serde::{Deserialize, Serialize};
use std::{fs, io::{self, Write}};
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureConfig {
    pub preg: bool,
    pub zdr: bool,
    pub who_asked: bool
}

impl Default for FeatureConfig {
    fn default() -> Self {
        Self {
            preg: true,
            zdr: true,
            who_asked: true
        }
    }
}

impl FeatureConfig {
    pub fn load(path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = fs::File::create(path)?;
        file.write_all(json.as_bytes())
    }
}

pub type SharedConfig = Arc<Mutex<FeatureConfig>>;

pub struct FeatureKey;

impl TypeMapKey for FeatureKey {
    type Value = SharedConfig;
}
