fn percentage_within_range(percentage: u32, start_range: u32, end_range: u32) -> u32 {
    // Handle edge cases: 0% should return exactly start_range, 100% should return
    // exactly end_range
    if percentage == 0 {
        return start_range;
    }
    if percentage == 100 {
        return end_range;
    }

    // For values between 0 and 100, use ceiling to ensure 1% is always > 0%
    // This fixes the bug where small percentages would round back to the minimum
    let range = end_range as f64 - start_range as f64;
    let result = (percentage as f64 / 100.0) * range + start_range as f64;
    result.ceil() as u32
}

fn get_is_on_text(is_on: bool) -> &'static str {
    if is_on {
        "On"
    } else {
        "Off"
    }
}

fn get_is_on_emoji(is_on: bool) -> &'static str {
    if is_on {
        "💡"
    } else {
        "🌑"
    }
}

fn get_is_back_on_emoji(is_on: bool) -> &'static str {
    if is_on {
        "🌈"
    } else {
        "🌑"
    }
}

fn check_device_filters<'a>(
    _context: &'a Litra,
    _serial_number: Option<&'a str>,
    device_path: Option<&'a str>,
    device_type: Option<&'a DeviceType>,
) -> impl Fn(&Device) -> bool + 'a {
    move |device| {
        // Check device path if specified
        if let Some(path) = device_path {
            return device.device_path() == path;
        }

        // Check device type if specified
        if let Some(expected_type) = device_type {
            if device.device_type() != *expected_type {
                return false;
            }
        }

        // If a serial number is specified, we'll filter by it after opening the device
        // since serial numbers are only accessible after opening
        true
    }
}

