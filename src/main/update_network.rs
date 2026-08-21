/// Environment variable to disable update checks
const DISABLE_UPDATE_CHECK_ENV: &str = "LITRA_DISABLE_UPDATE_CHECK";

/// Checks for updates by fetching releases from GitHub.
/// Returns the latest version tag if a newer version is available, None
/// otherwise. This function will timeout after 2 seconds and log a warning, but
/// will not disrupt the CLI's normal operation.
/// Set the LITRA_DISABLE_UPDATE_CHECK environment variable to any value to
/// disable this check. The check is performed at most once per day, with the
/// last check time stored in ~/.litra.toml. Only releases that are at least 72
/// hours old are considered.
fn check_for_updates() -> Option<String> {
    // Check if update check is disabled via environment variable
    if std::env::var(DISABLE_UPDATE_CHECK_ENV).is_ok() {
        return None;
    }

    // Read config and check if we should perform the update check
    let mut config = read_config();
    if !should_check_for_updates(&config) {
        return None;
    }

    // Update the last check timestamp regardless of the result
    config.update_check.last_check_timestamp = Some(current_timestamp());
    write_config(&config);

    let timeout = Duration::from_secs(UPDATE_CHECK_TIMEOUT_SECS);

    let agent = ureq::Agent::new_with_config(
        ureq::Agent::config_builder().timeout_global(Some(timeout)).build(),
    );

    let mut response = match agent
        .get(GITHUB_API_URL)
        .header("User-Agent", format!("litra-rs/{}", CURRENT_VERSION))
        .header("Accept", "application/vnd.github.v3+json")
        .call()
    {
        Ok(response) => response,
        Err(e) => {
            if let ureq::Error::Timeout(_) = e {
                eprintln!(
                    "Warning: Update check timed out after {} seconds",
                    UPDATE_CHECK_TIMEOUT_SECS
                );
            }
            // Silently ignore other errors to not disrupt CLI operation
            return None;
        },
    };

    let releases: Vec<GitHubRelease> = match response.body_mut().read_json() {
        Ok(releases) => releases,
        Err(_) => return None,
    };

    // Find the newest release that is at least 72 hours old and newer than the
    // current version Releases are sorted by date (newest first), but we need
    // the highest version that's old enough
    let mut best_version: Option<String> = None;

    for release in releases {
        // Skip releases that are too new (less than 72 hours old)
        if !is_release_old_enough(&release.published_at) {
            continue;
        }

        // Extract version from tag_name (e.g., "v3.2.0" -> "3.2.0")
        let release_version = release.tag_name.trim_start_matches('v');

        // Check if this release is newer than the current version
        if is_newer_version(release_version, CURRENT_VERSION) {
            // Check if this is better than our current best
            match &best_version {
                None => best_version = Some(release.tag_name),
                Some(current_best) => {
                    let current_best_version = current_best.trim_start_matches('v');
                    if is_newer_version(release_version, current_best_version) {
                        best_version = Some(release.tag_name);
                    }
                },
            }
        }
    }

    best_version
}

/// Compares two semantic version strings to determine if `latest` is newer than
/// `current`. Returns true if `latest` is a newer version.
fn is_newer_version(latest: &str, current: &str) -> bool {
    let parse_version = |v: &str| -> Option<(u32, u32, u32)> {
        let parts: Vec<&str> = v.split('.').collect();
        if parts.len() >= 3 {
            Some((parts[0].parse().ok()?, parts[1].parse().ok()?, parts[2].parse().ok()?))
        } else if parts.len() == 2 {
            Some((parts[0].parse().ok()?, parts[1].parse().ok()?, 0))
        } else if parts.len() == 1 {
            Some((parts[0].parse().ok()?, 0, 0))
        } else {
            None
        }
    };

    match (parse_version(latest), parse_version(current)) {
        (Some((l_major, l_minor, l_patch)), Some((c_major, c_minor, c_patch))) => {
            (l_major, l_minor, l_patch) > (c_major, c_minor, c_patch)
        },
        _ => false,
    }
}

/// Generates the update notification message for the given version
fn format_update_message(latest_version: &str) -> String {
    format!(
        "A new version of litra is available: {} (current: v{})\n\
         If you installed Litra from Homebrew, you can upgrade by running `brew upgrade litra`\n\
         Otherwise, you can download the latest release at  https://github.com/timrogers/litra-rs/releases/tag/{}",
        latest_version, CURRENT_VERSION, latest_version
    )
    .green()
    .to_string()
}
