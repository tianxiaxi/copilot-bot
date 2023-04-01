#[macro_use]
extern crate lazy_static;

pub mod i18n;
pub mod config;

// Load configuration and i18n from config file
lazy_static! {
    pub static ref I18N: i18n::I18n = i18n::load_i18n();

    pub static ref APPCONF: config::Config = config::load_config();
}
