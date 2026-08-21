#[derive(Debug)]
enum CliError {
    DeviceError(DeviceError),
    SerializationFailed(serde_json::Error),
    DeviceNotFound,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::DeviceError(error) => error.fmt(f),
            CliError::SerializationFailed(error) => error.fmt(f),
            CliError::DeviceNotFound => write!(f, "Device not found."),
        }
    }
}

impl From<DeviceError> for CliError {
    fn from(error: DeviceError) -> Self {
        CliError::DeviceError(error)
    }
}

type CliResult = Result<(), CliError>;

/// Get all devices matching the given filters
fn get_all_supported_devices(
    context: &Litra,
    serial_number: Option<&str>,
    device_path: Option<&str>,
    device_type: Option<&DeviceType>,
) -> Result<Vec<DeviceHandle>, CliError> {
    // Filter by various criteria
    let potential_devices: Vec<Device> = context
        .get_connected_devices()
        .filter(check_device_filters(context, serial_number, device_path, device_type))
        .collect();

    // If we need to filter by serial, open devices and check
    if let Some(serial) = serial_number {
        let mut handles = Vec::new();
        for device in potential_devices {
            if let Ok(handle) = device.open(context) {
                if let Ok(Some(actual_serial)) = handle.serial_number() {
                    if actual_serial == serial {
                        handles.push(handle);
                    }
                }
            }
        }
        Ok(handles)
    } else {
        // No serial filter, include all devices that matched the other filters
        Ok(potential_devices.into_iter().filter_map(|dev| dev.open(context).ok()).collect())
    }
}

/// Apply a command to device(s)
fn with_device<F>(
    serial_number: Option<&str>,
    device_path: Option<&str>,
    device_type: Option<&DeviceType>,
    callback: F,
) -> CliResult
where
    F: Fn(&DeviceHandle) -> DeviceResult<()>,
{
    let context = Litra::new()?;

    let devices = get_all_supported_devices(&context, serial_number, device_path, device_type)?;
    if devices.is_empty() {
        return Err(CliError::DeviceNotFound);
    }

    for device_handle in devices {
        // Ignore device-specific errors (e.g. unsupported device type) but propagate
        // validation errors (e.g. invalid brightness) since those indicate user input
        // errors
        if let Err(e) = callback(&device_handle) {
            if !matches!(e, DeviceError::UnsupportedDeviceType) {
                return Err(e.into());
            }
        }
    }
    Ok(())
}

