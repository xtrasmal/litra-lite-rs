#[cfg(feature = "cli")]
fn handle_devices_command(json: bool) -> CliResult {
    let litra_devices = get_connected_devices()?;

    if json {
        println!(
            "{}",
            serde_json::to_string(&litra_devices).map_err(CliError::SerializationFailed)?
        );
        Ok(())
    } else {
        if litra_devices.is_empty() {
            println!("No Logitech Litra devices found");
        } else {
            let table = Table::new(&litra_devices);
            println!("{}", table);
        }

        Ok(())
    }
}

fn handle_on_command(
    serial_number: Option<&str>,
    device_path: Option<&str>,
    device_type: Option<&DeviceType>,
) -> CliResult {
    with_device(serial_number, device_path, device_type, |device_handle| device_handle.set_on(true))
}

fn handle_off_command(
    serial_number: Option<&str>,
    device_path: Option<&str>,
    device_type: Option<&DeviceType>,
) -> CliResult {
    with_device(serial_number, device_path, device_type, |device_handle| {
        device_handle.set_on(false)
    })
}

fn handle_toggle_command(
    serial_number: Option<&str>,
    device_path: Option<&str>,
    device_type: Option<&DeviceType>,
) -> CliResult {
    // Get context to work with devices
    let context = Litra::new()?;

    // Get all matched devices
    let devices = get_all_supported_devices(&context, serial_number, device_path, device_type)?;
    if devices.is_empty() {
        return Err(CliError::DeviceNotFound);
    }

    // Toggle each device individually
    for device_handle in devices {
        // Toggle each device individually, ignoring errors
        if let Ok(is_on) = device_handle.is_on() {
            let _ = device_handle.set_on(!is_on);
        }
    }
    Ok(())
}

