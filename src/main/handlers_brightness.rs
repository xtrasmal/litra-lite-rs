/// Create a general purpose function to handle brightness setting
fn with_brightness_setting<F>(
    serial_number: Option<&str>,
    device_path: Option<&str>,
    device_type: Option<&DeviceType>,
    brightness_fn: F,
) -> CliResult
where
    F: Fn(&DeviceHandle) -> Result<u16, DeviceError>,
{
    let context = Litra::new()?;

    // Get all matched devices
    let devices = get_all_supported_devices(&context, serial_number, device_path, device_type)?;
    if devices.is_empty() {
        return Err(CliError::DeviceNotFound);
    }

    for device_handle in devices {
        if let Ok(brightness) = brightness_fn(&device_handle) {
            let _ = device_handle.set_brightness_in_lumen(brightness);
        }
    }
    Ok(())
}

fn handle_brightness_command(
    serial_number: Option<&str>,
    device_path: Option<&str>,
    device_type: Option<&DeviceType>,
    value: Option<u16>,
    percentage: Option<u8>,
) -> CliResult {
    match (value, percentage) {
        (Some(brightness), None) => {
            with_device(serial_number, device_path, device_type, |device_handle| {
                device_handle.set_brightness_in_lumen(brightness)
            })
        },
        (None, Some(pct)) => {
            with_brightness_setting(serial_number, device_path, device_type, |device_handle| {
                let brightness_in_lumen = percentage_within_range(
                    pct.into(),
                    device_handle.minimum_brightness_in_lumen().into(),
                    device_handle.maximum_brightness_in_lumen().into(),
                );

                // Convert to u16, handling any potential conversion errors
                // DeviceError doesn't have a constructor for this error type,
                // so we'll use InvalidBrightness as the closest match
                brightness_in_lumen.try_into().map_err(|_| DeviceError::InvalidBrightness(0))
            })
        },
        _ => unreachable!(),
    }
}

fn handle_brightness_up_command(
    serial_number: Option<&str>,
    device_path: Option<&str>,
    device_type: Option<&DeviceType>,
    value: Option<u16>,
    percentage: Option<u8>,
) -> CliResult {
    match (value, percentage) {
        (Some(brightness_to_add), None) => {
            with_brightness_setting(serial_number, device_path, device_type, |device_handle| {
                let current_brightness = device_handle.brightness_in_lumen()?;
                let new_brightness = current_brightness + brightness_to_add;
                Ok(new_brightness)
            })
        },
        (None, Some(pct)) => {
            with_brightness_setting(serial_number, device_path, device_type, |device_handle| {
                let current_brightness = device_handle.brightness_in_lumen()?;
                let brightness_to_add = percentage_within_range(
                    pct.into(),
                    device_handle.minimum_brightness_in_lumen().into(),
                    device_handle.maximum_brightness_in_lumen().into(),
                ) as u16
                    - device_handle.minimum_brightness_in_lumen();

                let new_brightness = current_brightness + brightness_to_add;
                Ok(new_brightness)
            })
        },
        _ => unreachable!(),
    }
}

fn handle_brightness_down_command(
    serial_number: Option<&str>,
    device_path: Option<&str>,
    device_type: Option<&DeviceType>,
    value: Option<u16>,
    percentage: Option<u8>,
) -> CliResult {
    match (value, percentage) {
        (Some(brightness_to_subtract), None) => {
            with_brightness_setting(serial_number, device_path, device_type, |device_handle| {
                let current_brightness = device_handle.brightness_in_lumen()?;

                if current_brightness <= brightness_to_subtract {
                    // Skip this device by returning an error which will be ignored
                    return Err(DeviceError::InvalidBrightness(0));
                }

                let new_brightness = current_brightness - brightness_to_subtract;
                Ok(new_brightness)
            })
        },
        (None, Some(pct)) => {
            with_brightness_setting(serial_number, device_path, device_type, |device_handle| {
                let current_brightness = device_handle.brightness_in_lumen()?;

                let brightness_to_subtract = percentage_within_range(
                    pct.into(),
                    device_handle.minimum_brightness_in_lumen().into(),
                    device_handle.maximum_brightness_in_lumen().into(),
                ) as u16
                    - device_handle.minimum_brightness_in_lumen();

                let new_brightness = current_brightness as i16 - brightness_to_subtract as i16;

                if new_brightness <= 0 {
                    // Skip this device by returning an error which will be ignored
                    return Err(DeviceError::InvalidBrightness(0));
                }

                Ok(new_brightness as u16)
            })
        },
        _ => unreachable!(),
    }
}

