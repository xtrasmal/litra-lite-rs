//! Read-only HID++ FeatureSet probe. It emits no state-changing commands.
use hidapi::HidApi;

const REPORT_SIZE: usize = 20;

fn packet(feature: u8, function: u8, parameter: u8) -> [u8; REPORT_SIZE] {
    let mut report = [0; REPORT_SIZE];
    report[..5].copy_from_slice(&[0x11, 0xff, feature, function, parameter]);
    report
}

fn packet_with_parameters(feature: u8, function: u8, parameters: &[u8]) -> [u8; REPORT_SIZE] {
    let mut report = packet(feature, function, 0);
    report[4..4 + parameters.len()].copy_from_slice(parameters);
    report
}

fn request(
    device: &hidapi::HidDevice,
    report: [u8; REPORT_SIZE],
) -> Result<[u8; REPORT_SIZE], Box<dyn std::error::Error>> {
    if device.write(&report)? != REPORT_SIZE {
        return Err("incomplete HID++ request write".into());
    }
    let mut response = [0; REPORT_SIZE];
    let count = device.read_timeout(&mut response, 1_000)?;
    if count < 4 {
        return Err("short or missing HID++ response".into());
    }
    if response[..4] != report[..4] {
        return Err("HID++ response did not match the request".into());
    }
    Ok(response)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = HidApi::new()?;
    let info = api
        .device_list()
        .find(|info| {
            info.vendor_id() == 0x046d
                && matches!(info.product_id(), 0xb903 | 0xc903)
                && info.usage_page() == 0xff43
        })
        .ok_or("no Litra Beam LX HID++ interface found")?;
    let device = info.open_device(&api)?;

    // Root.GetFeature(0x0001) finds FeatureSet dynamically.
    let mut root_lookup = packet(0, 0x0c, 0);
    root_lookup[4] = 0;
    root_lookup[5] = 1;
    let feature_set_index = request(&device, root_lookup)?[4];
    if feature_set_index == 0 {
        return Err("device does not advertise HID++ FeatureSet".into());
    }

    let count = request(&device, packet(feature_set_index, 0x0c, 0))?[4];
    println!("Beam LX HID++ FeatureSet: {count} features");
    println!("INDEX  FEATURE  TYPE  VERSION");
    for index in 0..count {
        let response = request(&device, packet(feature_set_index, 0x1c, index))?;
        println!(
            "{index:02x}     {:04x}     {:02x}    {:02x}",
            u16::from_be_bytes([response[4], response[5]]),
            response[6],
            response[7]
        );
    }

    // The following are documented information queries only. No light state is
    // changed and no software-control mode is claimed.
    let back_brightness = request(&device, packet(0x0a, 0x0c, 0))?;
    let max = u16::from_be_bytes([back_brightness[4], back_brightness[5]]);
    let min = u16::from_be_bytes([back_brightness[8], back_brightness[9]]);
    println!(
        "Rear brightness (0x8040): min={min}, max={max}, capabilities=0x{:02x}",
        back_brightness[7]
    );

    let rgb_info = request(&device, packet_with_parameters(0x0b, 0x0c, &[0xff, 0xff, 0]))?;
    println!(
        "RGB effects (0x8071): clusters={}, NV capabilities=0x{:04x}, extended capabilities=0x{:04x}",
        rgb_info[6],
        u16::from_be_bytes([rgb_info[7], rgb_info[8]]),
        u16::from_be_bytes([rgb_info[9], rgb_info[10]])
    );
    for cluster in 0..rgb_info[6] {
        let cluster_info =
            request(&device, packet_with_parameters(0x0b, 0x0c, &[cluster, 0xff, 0]))?;
        println!(
            "  cluster {cluster}: location=0x{:04x}, effects={}, display-persistence=0x{:02x}, effect-persistence=0x{:02x}",
            u16::from_be_bytes([cluster_info[6], cluster_info[7]]),
            cluster_info[8],
            cluster_info[9],
            cluster_info[10]
        );
        for effect in 0..cluster_info[8] {
            let effect_info =
                request(&device, packet_with_parameters(0x0b, 0x0c, &[cluster, effect, 0]))?;
            println!(
                "    effect {effect}: id=0x{:04x}, capabilities=0x{:04x}, period={}ms",
                u16::from_be_bytes([effect_info[6], effect_info[7]]),
                u16::from_be_bytes([effect_info[8], effect_info[9]]),
                u16::from_be_bytes([effect_info[10], effect_info[11]])
            );
        }
    }
    Ok(())
}
