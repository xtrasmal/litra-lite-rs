#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum DeviceType {
    #[serde(rename = "glow")]
    LitraGlow,
    #[serde(rename = "beam")]
    LitraBeam,
    #[serde(rename = "beam_lx")]
    LitraBeamLX,
}
impl DeviceType {
    #[must_use]
    pub fn has_back_side(&self) -> bool {
        *self == Self::LitraBeamLX
    }
}
impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LitraGlow => write!(f, "Litra Glow"),
            Self::LitraBeam => write!(f, "Litra Beam"),
            Self::LitraBeamLX => write!(f, "Litra Beam LX"),
        }
    }
}
impl std::str::FromStr for DeviceType {
    type Err = DeviceError;
    fn from_str(v: &str) -> DeviceResult<Self> {
        match v.to_lowercase().replace(' ', "").as_str() {
            "glow" => Ok(Self::LitraGlow),
            "beam" => Ok(Self::LitraBeam),
            "beam_lx" => Ok(Self::LitraBeamLX),
            _ => Err(DeviceError::UnsupportedDeviceType),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Brightness(u16);
impl Brightness {
    #[must_use]
    pub fn lumen(self) -> u16 {
        self.0
    }
}
impl TryFrom<u16> for Brightness {
    type Error = DeviceError;
    fn try_from(v: u16) -> DeviceResult<Self> {
        if (20..=400).contains(&v) {
            Ok(Self(v))
        } else {
            Err(DeviceError::InvalidBrightness(v))
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Temperature(u16);
impl Temperature {
    #[must_use]
    pub fn kelvin(self) -> u16 {
        self.0
    }
}
impl TryFrom<u16> for Temperature {
    type Error = DeviceError;
    fn try_from(v: u16) -> DeviceResult<Self> {
        if (MIN_TEMPERATURE..=MAX_TEMPERATURE).contains(&v) && v.is_multiple_of(100) {
            Ok(Self(v))
        } else {
            Err(DeviceError::InvalidTemperature(v))
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackBrightness(u8);
impl TryFrom<u8> for BackBrightness {
    type Error = DeviceError;
    fn try_from(v: u8) -> DeviceResult<Self> {
        if (1..=100).contains(&v) {
            Ok(Self(v))
        } else {
            Err(DeviceError::InvalidPercentage(v))
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zone(u8);
impl TryFrom<u8> for Zone {
    type Error = DeviceError;
    fn try_from(v: u8) -> DeviceResult<Self> {
        if (1..=7).contains(&v) {
            Ok(Self(v))
        } else {
            Err(DeviceError::InvalidZone(v))
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}
impl From<(u8, u8, u8)> for Rgb {
    fn from((red, green, blue): (u8, u8, u8)) -> Self {
        Self { red, green, blue }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Power(bool);
impl From<bool> for Power {
    fn from(v: bool) -> Self {
        Self(v)
    }
}

