use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    // config settings
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ..Default::default()
        }
    }
}


