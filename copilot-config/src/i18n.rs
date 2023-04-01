//! multiple language support.
//! Currently only supported: en, zh
use std::{fs::File, io::Read, path::PathBuf};

use copilot_utils::file;
use serde::{Serialize, Deserialize};

use crate::APPCONF;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct I18n {

}

impl Default for I18n {
    fn default() -> Self {
        Self { }
    }
}

pub fn load_i18n() -> I18n {
    let mut f = File::open(get_i18n_file()).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    toml::from_str(&contents).expect("Parse lane.xx.toml file failed!")
}

/// Get the i18n language config file
fn get_i18n_file() -> PathBuf {
    let i18n_file = format!("config/i18n/lang.{}.toml", APPCONF.server.language);
    file::get_execute_path().join(i18n_file)
}