//! The version information for copilot-bot

const HOMEPAGE: &str = "https://github.com/tianxiaxi/copilot-bot";

#[derive(Debug)]
pub struct CommitInfo {
    pub short_commit_hash: String,
    pub commit_hash: String,
    pub commit_date: String,
}

#[derive(Debug)]
pub struct VersionInfo {
    pub version: String,

    pub homepage: String,

    pub commit_info: CommitInfo,
}

/// Returns information about copilot's version.
pub fn version() -> VersionInfo {
    macro_rules! option_env_str {
        ($name:expr) => {
            option_env!($name).map(|s| s.to_string())
        };
    }

    let version = option_env_str!("CARGO_PKG_VERSION").unwrap_or_else(|| "0.0.0".to_string());
    let commit_info = CommitInfo {
        short_commit_hash: option_env_str!("COPILOT_BOT_COMMIT_SHORT_HASH").unwrap(),
        commit_hash: option_env_str!("COPILOT_BOT_COMMIT_HASH").unwrap(),
        commit_date: option_env_str!("COPILOT_BOT_COMMIT_DATE").unwrap(),
    };

    VersionInfo {
        version,
        homepage: HOMEPAGE.to_string(),
        commit_info,
    }
}
