#[test]
fn test_should_check_for_updates_never_checked() {
    // Should check if never checked before (no timestamp)
    let config = Config::default();
    assert!(should_check_for_updates(&config));
}

#[test]
fn test_should_check_for_updates_checked_recently() {
    // Should not check if checked less than a day ago
    let mut config = Config::default();
    config.update_check.last_check_timestamp = Some(current_timestamp());
    assert!(!should_check_for_updates(&config));
}

#[test]
fn test_should_check_for_updates_checked_long_ago() {
    // Should check if checked more than a day ago
    let mut config = Config::default();
    // Set timestamp to more than a day ago
    config.update_check.last_check_timestamp = Some(current_timestamp() - SECONDS_PER_DAY - 1);
    assert!(should_check_for_updates(&config));
}

#[test]
fn test_should_check_for_updates_exactly_one_day() {
    // Should check if exactly one day has passed
    let mut config = Config::default();
    config.update_check.last_check_timestamp = Some(current_timestamp() - SECONDS_PER_DAY);
    assert!(should_check_for_updates(&config));
}

#[test]
fn test_is_release_old_enough() {
    // A release from far in the past should be old enough
    assert!(is_release_old_enough("2020-01-01T00:00:00Z"));

    // A release from far in the future should not be old enough
    assert!(!is_release_old_enough("2099-01-01T00:00:00Z"));

    // An invalid timestamp should not be old enough (returns false)
    assert!(!is_release_old_enough("invalid"));
}

#[test]
fn test_format_update_message() {
    let message = format_update_message("v3.3.0");
    assert!(message.contains("v3.3.0"));
    assert!(message.contains(CURRENT_VERSION));
    assert!(message.contains("https://github.com/timrogers/litra-rs/releases/tag/v3.3.0"));
}
