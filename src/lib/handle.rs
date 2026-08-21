pub struct DeviceHandle {
    transport: Box<dyn HidTransport>,
    device_type: DeviceType,
    device_path: String,
    serial_number: Option<String>,
    features: Features,
}
impl fmt::Debug for DeviceHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeviceHandle")
            .field("device_type", &self.device_type)
            .field("device_path", &self.device_path)
            .finish_non_exhaustive()
    }
}
impl DeviceHandle {
    fn new(
        t: Box<dyn HidTransport>,
        dt: DeviceType,
        path: String,
        serial: Option<String>,
    ) -> DeviceResult<Self> {
        let illumination = resolve_feature(t.as_ref(), ILLUMINATION_FEATURE)?;
        if illumination == 0 {
            return Err(DeviceError::UnsupportedFeature("Illumination (0x1990)"));
        }
        let rear_brightness = resolve_feature(t.as_ref(), BRIGHTNESS_CONTROL_FEATURE)
            .ok()
            .filter(|index| *index != 0);
        let rear_colour =
            resolve_feature(t.as_ref(), PER_KEY_LIGHTING_FEATURE).ok().filter(|index| *index != 0);
        Ok(Self {
            transport: t,
            device_type: dt,
            device_path: path,
            serial_number: serial,
            features: Features { illumination, rear_brightness, rear_colour },
        })
    }
    #[must_use]
    pub fn device_type(&self) -> DeviceType {
        self.device_type
    }
    pub fn serial_number(&self) -> DeviceResult<Option<String>> {
        Ok(self.serial_number.clone())
    }
    pub fn device_path(&self) -> DeviceResult<String> {
        Ok(self.device_path.clone())
    }
    pub fn is_on(&self) -> DeviceResult<bool> {
        Ok(self.call(Command::GetPower)?[4] == 1)
    }
    pub fn set_on(&self, v: bool) -> DeviceResult<()> {
        self.call(Command::SetPower(v.into())).map(|_| ())
    }
    pub fn brightness_in_lumen(&self) -> DeviceResult<u16> {
        let r = self.call(Command::GetBrightness)?;
        ensure_len(&r, 6)?;
        Ok(u16::from_be_bytes([r[4], r[5]]))
    }
    pub fn set_brightness_in_lumen(&self, v: u16) -> DeviceResult<()> {
        let v = Brightness::try_from(v)?;
        if v.0 < self.minimum_brightness_in_lumen() || v.0 > self.maximum_brightness_in_lumen() {
            return Err(DeviceError::InvalidBrightness(v.0));
        }
        self.call(Command::SetBrightness(v)).map(|_| ())
    }
    #[must_use]
    pub fn minimum_brightness_in_lumen(&self) -> u16 {
        match self.device_type {
            DeviceType::LitraGlow => 20,
            DeviceType::LitraBeam | DeviceType::LitraBeamLX => 30,
        }
    }
    #[must_use]
    pub fn maximum_brightness_in_lumen(&self) -> u16 {
        match self.device_type {
            DeviceType::LitraGlow => 250,
            DeviceType::LitraBeam | DeviceType::LitraBeamLX => 400,
        }
    }
    pub fn temperature_in_kelvin(&self) -> DeviceResult<u16> {
        let r = self.call(Command::GetTemperature)?;
        ensure_len(&r, 6)?;
        Ok(u16::from_be_bytes([r[4], r[5]]))
    }
    pub fn set_temperature_in_kelvin(&self, v: u16) -> DeviceResult<()> {
        self.call(Command::SetTemperature(Temperature::try_from(v)?)).map(|_| ())
    }
    #[must_use]
    pub fn minimum_temperature_in_kelvin(&self) -> u16 {
        MIN_TEMPERATURE
    }
    #[must_use]
    pub fn maximum_temperature_in_kelvin(&self) -> u16 {
        MAX_TEMPERATURE
    }
    pub fn set_back_color(&self, zone: u8, red: u8, green: u8, blue: u8) -> DeviceResult<()> {
        let Zone(zone) = Zone::try_from(zone)?;
        let Rgb { red, green, blue } = Rgb::from((red, green, blue));
        let feature = self
            .features
            .rear_colour
            .ok_or(DeviceError::UnsupportedFeature("Per-key Lighting v2 (0x8081)"))?;
        let colour = [
            REPORT_ID,
            DEVICE_INDEX,
            feature,
            0x1b,
            zone,
            red.max(1),
            green.max(1),
            blue.max(1),
            0xff,
            0,
            0,
            0,
            0xff,
            0,
            0,
            0,
            0xff,
            0,
            0,
            0,
        ];
        write_known(self.transport.as_ref(), &colour)?;
        write_known(
            self.transport.as_ref(),
            &[REPORT_ID, DEVICE_INDEX, feature, 0x7b, 0, 0, 1, 0, 0],
        )
    }
    pub fn set_back_brightness_percentage(&self, value: u8) -> DeviceResult<()> {
        self.call_rear(Command::SetRearBrightness(BackBrightness::try_from(value)?)).map(|_| ())
    }
    pub fn set_back_on(&self, value: bool) -> DeviceResult<()> {
        self.call_rear(Command::SetRearPower(value.into())).map(|_| ())
    }
    pub fn is_back_on(&self) -> DeviceResult<bool> {
        Ok(self.call_rear(Command::GetRearPower)?[4] & 1 == 1)
    }
    pub fn back_brightness_percentage(&self) -> DeviceResult<u8> {
        let response = self.call_rear(Command::GetRearBrightness)?;
        ensure_len(&response, 6)?;
        u8::try_from(u16::from_be_bytes([response[4], response[5]]))
            .map_err(|_| DeviceError::UnexpectedResponse)
    }
    fn call(&self, c: Command) -> DeviceResult<[u8; REPORT_SIZE]> {
        request(self.transport.as_ref(), c.packet(self.features.illumination))
    }
    fn call_rear(&self, c: Command) -> DeviceResult<[u8; REPORT_SIZE]> {
        let feature = self
            .features
            .rear_brightness
            .ok_or(DeviceError::UnsupportedFeature("Brightness Control (0x8040)"))?;
        request(self.transport.as_ref(), c.packet(feature))
    }
}
