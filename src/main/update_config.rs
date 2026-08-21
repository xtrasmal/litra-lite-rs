/// The current version of the CLI, extracted from Cargo.toml
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// GitHub API URL for fetching releases (list endpoint)
const GITHUB_API_URL: &str = "https://api.github.com/repos/timrogers/litra-rs/releases";

/// Timeout for update check requests in seconds
const UPDATE_CHECK_TIMEOUT_SECS: u64 = 2;

/// Response structure for GitHub releases API
#[derive(serde::Deserialize)]
struct GitHubRelease {
    tag_name: String,
    published_at: String,
}

/// Configuration file name
const CONFIG_FILE_NAME: &str = ".litra.toml";

/// Number of seconds in a day (24 hours)
const SECONDS_PER_DAY: u64 = 86400;

/// Configuration structure for litra.toml
#[derive(serde::Deserialize, serde::Serialize, Default)]
struct Config {
    #[serde(default)]
    update_check: UpdateCheckConfig,
}

/// Update check configuration
#[derive(serde::Deserialize, serde::Serialize, Default)]
struct UpdateCheckConfig {
    /// Unix timestamp of the last update check
    last_check_timestamp: Option<u64>,
}

/// Returns the path to the litra.toml config file in the user's home directory
#[cfg(feature = "cli")]
fn get_config_path() -> Option<std::path::PathBuf> {
    dirs::home_dir().map(|home| home.join(CONFIG_FILE_NAME))
}

/// Reads the config file, returning a default config if the file doesn't exist
/// or can't be read
#[cfg(feature = "cli")]
fn read_config() -> Config {
    let Some(config_path) = get_config_path() else {
        return Config::default();
    };

    match std::fs::read_to_string(&config_path) {
        Ok(contents) => toml::from_str(&contents).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

/// Writes the config to the config file, silently ignoring errors
#[cfg(feature = "cli")]
fn write_config(config: &Config) {
    let Some(config_path) = get_config_path() else {
        return;
    };

    if let Ok(contents) = toml::to_string_pretty(config) {
        let _ = std::fs::write(&config_path, contents);
    }
}

/// Returns the current Unix timestamp in seconds
#[cfg(feature = "cli")]
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Checks if enough time has passed since the last update check (at least one
/// day)
#[cfg(feature = "cli")]
fn should_check_for_updates(config: &Config) -> bool {
    let Some(last_check) = config.update_check.last_check_timestamp else {
        return true; // Never checked before
    };

    let now = current_timestamp();
    now.saturating_sub(last_check) >= SECONDS_PER_DAY
}

/// Checks if a release is old enough to be considered for update notifications
/// (at least 72 hours) Uses chrono for ISO 8601 parsing and comparison
#[cfg(feature = "cli")]
fn is_release_old_enough(published_at: &str) -> bool {
    use chrono::{DateTime, Duration, Utc};

    // Parse the release timestamp
    let Ok(release_time) = DateTime::parse_from_rfc3339(published_at) else {
        return false; // If we can't parse the timestamp, skip this release
    };

    // Calculate the cutoff time (72 hours ago)
    let cutoff = Utc::now() - Duration::hours(72);

    // Check if the release is older than the cutoff
    release_time < cutoff
}
