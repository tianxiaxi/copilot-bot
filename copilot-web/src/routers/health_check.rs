//! # health_check
//!
//! The crate `health_check` only detect the web server health
//! and would not contain any logic bussiness.

/// `check_server_health` return the health status
/// mostly return the `success` message if it is running.
pub async fn check_server_health() -> String {
    "success".to_owned()
}

/// `welcome` indicates the homepage.
/// But not we do not have any frontend framework and always return the welcome message.
pub async fn welcome() -> String {
    "Wellcome to my eden!".to_owned()
}
