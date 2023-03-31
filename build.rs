use std::env;
use std::fs;
use std::path::Path;

fn main() {
    commit_info();
}

/// get git commit info
fn commit_info() {
    if !Path::new(".git").exists() {
        return;
    }
    /// command: git log -1 --abbrev=8 --date=short --format='%H %h %cd'
    let output = match Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--date=short")
        .arg("--format=%H %h %cd")
        .arg("--abbrev=9")
        .output()
    {
        Ok(output) if output.status.success() => output,
        _ => return,
    };
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut parts = stdout.split_whitespace();
    let mut next = || parts.next().unwrap();
    println!("cargo:rustc-env=COPILOT_BOT_COMMIT_HASH={}", next());
    println!("cargo:rustc-env=COPILOT_BOT_COMMIT_SHORT_HASH={}", next());
    println!("cargo:rustc-env=COPILOT_BOT_COMMIT_DATE={}", next());
}
