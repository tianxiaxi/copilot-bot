//! Configuration file

use std::{fs::File, io::Read};

use serde::{Serialize, Deserialize};
use copilot_utils::file;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    pub host: String,
    pub port: u32,
    pub language: String,
}

impl Default for Server {
    fn default() -> Self {
        Self { host: "127.0.0.0.1".to_string(), port: 3000, language: "en".to_string() }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Telegram {

}

impl Default for Telegram {
    fn default() -> Self {
        Self {  }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAI {

}

impl Default for OpenAI {
    fn default() -> Self {
        Self {  }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub server: Server,
    pub telegram: Telegram,
    pub openai: OpenAI,
}

impl Default for Config {
    fn default() -> Self {
        Self { server: Server::default(), telegram: Telegram::default(), openai: OpenAI::default() }
    }
}

/// load config file
pub fn load_config() -> Config {
    let path = file::get_execute_path().join("config/app.toml");
    let mut f = File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    toml::from_str(&contents).expect("Parse app.toml file failed!")
}
