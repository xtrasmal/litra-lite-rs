#[test]
fn test_is_newer_version_major() {
    assert!(is_newer_version("4.0.0", "3.2.0"));
    assert!(is_newer_version("2.0.0", "1.0.0"));
    assert!(!is_newer_version("1.0.0", "2.0.0"));
    assert!(!is_newer_version("3.0.0", "4.0.0"));
}

#[test]
fn test_is_newer_version_minor() {
    assert!(is_newer_version("3.3.0", "3.2.0"));
    assert!(is_newer_version("1.2.0", "1.1.0"));
    assert!(!is_newer_version("1.1.0", "1.2.0"));
    assert!(!is_newer_version("3.2.0", "3.3.0"));
}

#[test]
fn test_is_newer_version_patch() {
    assert!(is_newer_version("3.2.1", "3.2.0"));
    assert!(is_newer_version("1.0.5", "1.0.4"));
    assert!(!is_newer_version("1.0.4", "1.0.5"));
    assert!(!is_newer_version("3.2.0", "3.2.1"));
}

#[test]
fn test_is_newer_version_same_version() {
    assert!(!is_newer_version("3.2.0", "3.2.0"));
    assert!(!is_newer_version("1.0.0", "1.0.0"));
}

#[test]
fn test_is_newer_version_edge_cases() {
    // Two-part version
    assert!(is_newer_version("3.3", "3.2"));
    assert!(!is_newer_version("3.2", "3.3"));
    // One-part version
    assert!(is_newer_version("4", "3"));
    assert!(!is_newer_version("3", "4"));
    // Invalid version format
    assert!(!is_newer_version("invalid", "3.2.0"));
    assert!(!is_newer_version("3.2.0", "invalid"));
    assert!(!is_newer_version("", "3.2.0"));
}

