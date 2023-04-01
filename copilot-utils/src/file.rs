//! file/path utility

use std::path::PathBuf;

/// Get the execute file directory
/// # Examples
/// 
/// ```
/// let path = get_execute_path();
/// assert_eq!(path.is_dir(), true);
/// ```
pub fn get_execute_path() -> PathBuf {
    let path = std::env::current_exe().unwrap();
    let dir = path.parent().expect("Except a directory");
    dir.to_path_buf()
}
