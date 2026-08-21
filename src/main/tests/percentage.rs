#[test]
fn test_percentage_within_range_zero_percent() {
    // 0% should return exactly the minimum value
    assert_eq!(percentage_within_range(0, 20, 250), 20);
    assert_eq!(percentage_within_range(0, 30, 400), 30);
    assert_eq!(percentage_within_range(0, 100, 200), 100);
}

#[test]
fn test_percentage_within_range_hundred_percent() {
    // 100% should return exactly the maximum value
    assert_eq!(percentage_within_range(100, 20, 250), 250);
    assert_eq!(percentage_within_range(100, 30, 400), 400);
    assert_eq!(percentage_within_range(100, 100, 200), 200);
}

#[test]
fn test_percentage_within_range_one_percent_greater_than_zero() {
    // 1% should return a value greater than 0%
    // This is the key bug fix: 1% should never equal the minimum
    let zero_pct = percentage_within_range(0, 20, 250);
    let one_pct = percentage_within_range(1, 20, 250);
    assert!(one_pct > zero_pct, "1% ({}) should be greater than 0% ({})", one_pct, zero_pct);

    // Test with Litra Beam range
    let zero_pct_beam = percentage_within_range(0, 30, 400);
    let one_pct_beam = percentage_within_range(1, 30, 400);
    assert!(
        one_pct_beam > zero_pct_beam,
        "1% ({}) should be greater than 0% ({})",
        one_pct_beam,
        zero_pct_beam
    );

    // Test with a small range where the bug is most apparent
    let zero_pct_small = percentage_within_range(0, 20, 30);
    let one_pct_small = percentage_within_range(1, 20, 30);
    assert!(
        one_pct_small > zero_pct_small,
        "1% ({}) should be greater than 0% ({}) even with small range",
        one_pct_small,
        zero_pct_small
    );
}

#[test]
fn test_percentage_within_range_midpoint() {
    // 50% should return approximately the midpoint
    // For range 20-250: midpoint is 135
    let result = percentage_within_range(50, 20, 250);
    assert_eq!(result, 135);

    // For range 30-400: midpoint is 215
    let result = percentage_within_range(50, 30, 400);
    assert_eq!(result, 215);
}

#[test]
fn test_percentage_within_range_litra_glow() {
    // Test with Litra Glow's actual brightness range (20-250lm)
    assert_eq!(percentage_within_range(0, 20, 250), 20);
    assert!(percentage_within_range(1, 20, 250) > 20);
    assert_eq!(percentage_within_range(100, 20, 250), 250);

    // Verify that small percentages are distinguishable
    let one = percentage_within_range(1, 20, 250);
    let two = percentage_within_range(2, 20, 250);
    let three = percentage_within_range(3, 20, 250);

    // Each should be at least the minimum
    assert!(one >= 20);
    assert!(two >= 20);
    assert!(three >= 20);

    // And 1% should be greater than 0%
    assert!(one > 20);
}

#[test]
fn test_percentage_within_range_litra_beam() {
    // Test with Litra Beam's actual brightness range (30-400lm)
    assert_eq!(percentage_within_range(0, 30, 400), 30);
    assert!(percentage_within_range(1, 30, 400) > 30);
    assert_eq!(percentage_within_range(100, 30, 400), 400);

    // Verify that small percentages are distinguishable
    let one = percentage_within_range(1, 30, 400);
    let two = percentage_within_range(2, 30, 400);

    // Each should be greater than minimum
    assert!(one > 30);
    assert!(two > 30);
}

#[test]
fn test_percentage_within_range_small_range() {
    // Test with a small range where rounding issues are most apparent
    // This is the case that exposes the original bug
    let range_start = 20;
    let range_end = 30;

    assert_eq!(percentage_within_range(0, range_start, range_end), range_start);
    assert!(
        percentage_within_range(1, range_start, range_end) > range_start,
        "1% should be greater than start even with small range"
    );
    assert!(
        percentage_within_range(5, range_start, range_end) > range_start,
        "5% should be greater than start even with small range"
    );
    assert_eq!(percentage_within_range(100, range_start, range_end), range_end);
}

#[test]
fn test_percentage_within_range_monotonic() {
    // Verify that the function is monotonically increasing
    // (higher percentages should never give lower values)
    let range_start = 20;
    let range_end = 250;

    let mut prev_value = percentage_within_range(0, range_start, range_end);
    for pct in 1..=100 {
        let current_value = percentage_within_range(pct, range_start, range_end);
        assert!(
            current_value >= prev_value,
            "{}% ({}) should be >= {}% ({})",
            pct,
            current_value,
            pct - 1,
            prev_value
        );
        prev_value = current_value;
    }
}

