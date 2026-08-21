#[cfg_attr(feature = "cli", derive(Tabled))]
#[derive(Serialize, Debug)]
pub struct DeviceInfo {
    #[cfg_attr(feature = "cli", tabled(skip))]
    pub device_type: DeviceType,
    #[cfg_attr(feature = "cli", tabled(rename = "Type"))]
    pub device_type_display: String,
    #[cfg_attr(feature = "cli", tabled(skip))]
    pub has_back_side: bool,
    #[cfg_attr(feature = "cli", tabled(rename = "Serial Number"))]
    pub serial_number: String,
    #[cfg_attr(feature = "cli", tabled(rename = "Device Path"))]
    pub device_path: String,
    #[cfg_attr(feature = "cli", tabled(rename = "Status"))]
    pub status_display: String,
    #[cfg_attr(feature = "cli", tabled(rename = "Brightness (lm)"))]
    pub brightness_display: String,
    #[cfg_attr(feature = "cli", tabled(rename = "Temperature (K)"))]
    pub temperature_display: String,
    #[cfg_attr(feature = "cli", tabled(rename = "Back Status"))]
    pub back_status_display: String,
    #[cfg_attr(feature = "cli", tabled(rename = "Back Brightness (%)"))]
    pub back_brightness_display: String,
    // Keep original fields for JSON output
    #[cfg_attr(feature = "cli", tabled(skip))]
    pub is_on: bool,
    #[cfg_attr(feature = "cli", tabled(skip))]
    pub brightness_in_lumen: u16,
    #[cfg_attr(feature = "cli", tabled(skip))]
    pub temperature_in_kelvin: u16,
    #[cfg_attr(feature = "cli", tabled(skip))]
    pub minimum_brightness_in_lumen: u16,
    #[cfg_attr(feature = "cli", tabled(skip))]
    pub maximum_brightness_in_lumen: u16,
    #[cfg_attr(feature = "cli", tabled(skip))]
    pub minimum_temperature_in_kelvin: u16,
    #[cfg_attr(feature = "cli", tabled(skip))]
    pub maximum_temperature_in_kelvin: u16,
    #[cfg_attr(feature = "cli", tabled(skip))]
    pub is_back_on: Option<bool>,
    #[cfg_attr(feature = "cli", tabled(skip))]
    pub back_brightness_percentage: Option<u8>,
}

fn get_connected_devices() -> Result<Vec<DeviceInfo>, CliError> {
    let context = Litra::new()?;

    let litra_devices: Vec<DeviceInfo> = context
        .get_connected_devices()
        .filter_map(|device| {
            let device_handle = match device.open(&context) {
                Ok(handle) => handle,
                Err(_e) => {
                    return None;
                },
            };

            // Get the device path
            let device_path = device.device_path();

            // Get serial number if available
            let serial = match device_handle.serial_number() {
                Ok(Some(s)) => s,
                Ok(None) => "UNKNOWN".to_string(),
                Err(_e) => "UNKNOWN".to_string(),
            };

            // Try to get attributes, log errors
            let is_on = match device_handle.is_on() {
                Ok(on) => on,
                Err(_e) => {
                    return None;
                },
            };

            let brightness = match device_handle.brightness_in_lumen() {
                Ok(b) => b,
                Err(_e) => {
                    return None;
                },
            };

            let temperature = match device_handle.temperature_in_kelvin() {
                Ok(t) => t,
                Err(_e) => {
                    return None;
                },
            };

            // Get back light status for Litra Beam LX devices
            let (
                is_back_on,
                back_brightness_percentage,
                back_status_display,
                back_brightness_display,
            ) = if device.device_type() == DeviceType::LitraBeamLX {
                let back_on = device_handle.is_back_on().ok();
                let back_brightness = device_handle.back_brightness_percentage().ok();
                let status_display = match back_on {
                    Some(on) => format!("{} {}", get_is_on_text(on), get_is_back_on_emoji(on)),
                    None => "Unknown".to_string(),
                };
                let brightness_display = match back_brightness {
                    Some(b) => format!("{}%", b),
                    None => "Unknown".to_string(),
                };
                (back_on, back_brightness, status_display, brightness_display)
            } else {
                (None, None, "N/A".to_string(), "N/A".to_string())
            };

            Some(DeviceInfo {
                device_type: device.device_type(),
                device_type_display: device.device_type().to_string(),
                has_back_side: device.device_type().has_back_side(),
                serial_number: serial,
                device_path,
                status_display: format!("{} {}", get_is_on_text(is_on), get_is_on_emoji(is_on)),
                brightness_display: format!(
                    "{}/{}",
                    brightness,
                    device_handle.maximum_brightness_in_lumen()
                ),
                temperature_display: format!(
                    "{}/{}",
                    temperature,
                    device_handle.maximum_temperature_in_kelvin()
                ),
                back_status_display,
                back_brightness_display,
                // Keep original fields for JSON output
                is_on,
                brightness_in_lumen: brightness,
                temperature_in_kelvin: temperature,
                minimum_brightness_in_lumen: device_handle.minimum_brightness_in_lumen(),
                maximum_brightness_in_lumen: device_handle.maximum_brightness_in_lumen(),
                minimum_temperature_in_kelvin: device_handle.minimum_temperature_in_kelvin(),
                maximum_temperature_in_kelvin: device_handle.maximum_temperature_in_kelvin(),
                is_back_on,
                back_brightness_percentage,
            })
        })
        .collect();
    Ok(litra_devices)
}

