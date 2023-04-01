//! Configuration file

use std::{fs::File, io::Read};

use serde::{Serialize, Deserialize};
use copilot_utils::file;

fn default_language() -> String { "en".to_string() }

fn default_host() -> String { "127.0.0.1".to_string() }

fn default_port() -> u32 { 3000 }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u32,
    #[serde(default = "default_language")]
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
    toml::from_str(&contents)
    .expect("Parse app.toml file failed!")
}
