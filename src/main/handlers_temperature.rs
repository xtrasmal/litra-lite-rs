fn handle_temperature_command(
    serial_number: Option<&str>,
    device_path: Option<&str>,
    device_type: Option<&DeviceType>,
    value: u16,
) -> CliResult {
    with_device(serial_number, device_path, device_type, |device_handle| {
        device_handle.set_temperature_in_kelvin(value)
    })
}

fn handle_temperature_up_command(
    serial_number: Option<&str>,
    device_path: Option<&str>,
    device_type: Option<&DeviceType>,
    value: u16,
) -> CliResult {
    with_device(serial_number, device_path, device_type, |device_handle| {
        let current_temperature = device_handle.temperature_in_kelvin()?;
        let new_temperature = current_temperature + value;

        // Check if new temperature would exceed maximum
        if new_temperature > device_handle.maximum_temperature_in_kelvin() {
            return Err(DeviceError::InvalidTemperature(new_temperature));
        }

        device_handle.set_temperature_in_kelvin(new_temperature)
    })
}

fn handle_temperature_down_command(
    serial_number: Option<&str>,
    device_path: Option<&str>,
    device_type: Option<&DeviceType>,
    value: u16,
) -> CliResult {
    with_device(serial_number, device_path, device_type, |device_handle| {
        let current_temperature = device_handle.temperature_in_kelvin()?;

        // Check if new temperature would be below minimum
        if current_temperature <= value {
            // Skip this device by returning an error which will be ignored
            return Err(DeviceError::InvalidTemperature(0));
        }

        let new_temperature = current_temperature - value;
        device_handle.set_temperature_in_kelvin(new_temperature)
    })
}

fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), String> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return Err("Hex color must be exactly 6 characters long".into());
    }

    let r = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|_| "Failed to parse red component from hex color")?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|_| "Failed to parse green component from hex color")?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|_| "Failed to parse blue component from hex color")?;

    Ok((r, g, b))
}

