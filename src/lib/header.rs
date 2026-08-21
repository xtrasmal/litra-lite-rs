use hidapi::{DeviceInfo, HidApi, HidDevice, HidError};
use std::{error::Error, fmt};

const VENDOR_ID: u16 = 0x046d;
const USAGE_PAGE: u16 = 0xff43;
const REPORT_ID: u8 = 0x11;
const DEVICE_INDEX: u8 = 0xff;
const REPORT_SIZE: usize = 20;
const ERROR_FEATURE: u8 = 0x8f;
const SOFTWARE_ID: u8 = 0x0c;
const READ_TIMEOUT_MS: i32 = 1_000;
const ILLUMINATION_FEATURE: u16 = 0x1990;
const BRIGHTNESS_CONTROL_FEATURE: u16 = 0x8040;
const PER_KEY_LIGHTING_FEATURE: u16 = 0x8081;
const MIN_TEMPERATURE: u16 = 2700;
const MAX_TEMPERATURE: u16 = 6500;

pub struct Litra(HidApi);
impl Litra {
    pub fn new() -> DeviceResult<Self> {
        let api = HidApi::new()?;
        api.set_open_exclusive(false);
        Ok(Self(api))
    }
    pub fn get_connected_devices(&self) -> impl Iterator<Item = Device<'_>> {
        let mut v: Vec<_> = self.0.device_list().filter_map(|i| Device::try_from(i).ok()).collect();
        v.sort_by_key(|d| d.device_path());
        v.into_iter()
    }
    pub fn refresh_connected_devices(&mut self) -> DeviceResult<()> {
        self.0.refresh_devices()?;
        Ok(())
    }
}
