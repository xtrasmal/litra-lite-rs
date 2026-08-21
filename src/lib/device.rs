pub struct Device<'a> {
    device_info: &'a DeviceInfo,
    device_type: DeviceType,
}
impl<'a> TryFrom<&'a DeviceInfo> for Device<'a> {
    type Error = DeviceError;
    fn try_from(i: &'a DeviceInfo) -> DeviceResult<Self> {
        if i.vendor_id() != VENDOR_ID || i.usage_page() != USAGE_PAGE {
            return Err(DeviceError::Unsupported);
        };
        device_type_from_product_id(i.product_id())
            .map(|device_type| Self { device_info: i, device_type })
            .ok_or(DeviceError::Unsupported)
    }
}
impl Device<'_> {
    #[must_use]
    pub fn device_type(&self) -> DeviceType {
        self.device_type
    }
    #[must_use]
    pub fn device_path(&self) -> String {
        self.device_info.path().to_string_lossy().into_owned()
    }
    pub fn open(&self, c: &Litra) -> DeviceResult<DeviceHandle> {
        let serial =
            self.device_info.serial_number().filter(|v| !v.is_empty()).map(ToOwned::to_owned);
        DeviceHandle::new(
            Box::new(HidDeviceTransport(self.device_info.open_device(&c.0)?)),
            self.device_type,
            self.device_path(),
            serial,
        )
    }
}

