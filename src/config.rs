use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    temp: String,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}
