fn handle_back_color_command(
    serial_number: Option<&str>,
    device_path: Option<&str>,
    hex: &str,
    zone_id: Option<u8>,
) -> CliResult {
    with_device(serial_number, device_path, Some(&DeviceType::LitraBeamLX), |device_handle| {
        match hex_to_rgb(hex) {
            Ok((r, g, b)) => match zone_id {
                None => {
                    for i in 1..=7 {
                        device_handle.set_back_color(i, r, g, b)?;
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                    Ok(())
                },
                Some(id) => {
                    device_handle.set_back_color(id, r, g, b)?;
                    Ok(())
                },
            },
            Err(error) => Err(DeviceError::InvalidColor(error)),
        }
    })
}

fn handle_back_brightness_command(
    serial_number: Option<&str>,
    device_path: Option<&str>,
    brightness: u8,
) -> CliResult {
    with_device(serial_number, device_path, Some(&DeviceType::LitraBeamLX), |device_handle| {
        device_handle.set_back_brightness_percentage(brightness)
    })
}

fn handle_back_off_command(serial_number: Option<&str>, device_path: Option<&str>) -> CliResult {
    with_device(serial_number, device_path, Some(&DeviceType::LitraBeamLX), |device_handle| {
        device_handle.set_back_on(false)
    })
}

fn handle_back_on_command(serial_number: Option<&str>, device_path: Option<&str>) -> CliResult {
    with_device(serial_number, device_path, Some(&DeviceType::LitraBeamLX), |device_handle| {
        device_handle.set_back_on(true)
    })
}

fn handle_back_toggle_command(serial_number: Option<&str>, device_path: Option<&str>) -> CliResult {
    // Get context to work with devices
    let context = Litra::new()?;

    // Get all matched devices (only Litra Beam LX supports back light)
    let devices = get_all_supported_devices(
        &context,
        serial_number,
        device_path,
        Some(&DeviceType::LitraBeamLX),
    )?;
    if devices.is_empty() {
        return Err(CliError::DeviceNotFound);
    }

    // Toggle each device individually
    for device_handle in devices {
        // Toggle each device individually, ignoring errors
        if let Ok(is_on) = device_handle.is_back_on() {
            let _ = device_handle.set_back_on(!is_on);
        }
    }
    Ok(())
}

fn handle_back_brightness_up_command(
    serial_number: Option<&str>,
    device_path: Option<&str>,
    percentage: u8,
) -> CliResult {
    // Get context to work with devices
    let context = Litra::new()?;

    // Get all matched devices (only Litra Beam LX supports back light)
    let devices = get_all_supported_devices(
        &context,
        serial_number,
        device_path,
        Some(&DeviceType::LitraBeamLX),
    )?;
    if devices.is_empty() {
        return Err(CliError::DeviceNotFound);
    }

    for device_handle in devices {
        if let Ok(current_brightness) = device_handle.back_brightness_percentage() {
            let new_brightness = current_brightness.saturating_add(percentage).min(100);
            let _ = device_handle.set_back_brightness_percentage(new_brightness);
        }
    }
    Ok(())
}

fn handle_back_brightness_down_command(
    serial_number: Option<&str>,
    device_path: Option<&str>,
    percentage: u8,
) -> CliResult {
    // Get context to work with devices
    let context = Litra::new()?;

    // Get all matched devices (only Litra Beam LX supports back light)
    let devices = get_all_supported_devices(
        &context,
        serial_number,
        device_path,
        Some(&DeviceType::LitraBeamLX),
    )?;
    if devices.is_empty() {
        return Err(CliError::DeviceNotFound);
    }

    for device_handle in devices {
        if let Ok(current_brightness) = device_handle.back_brightness_percentage() {
            let new_brightness = current_brightness.saturating_sub(percentage).max(1);
            let _ = device_handle.set_back_brightness_percentage(new_brightness);
        }
    }
    Ok(())
}
