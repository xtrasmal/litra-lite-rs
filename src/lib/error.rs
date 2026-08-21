#[derive(Debug)]
pub enum DeviceError {
    Unsupported,
    InvalidBrightness(u16),
    InvalidTemperature(u16),
    InvalidPercentage(u8),
    HidError(HidError),
    UnsupportedDeviceType,
    InvalidZone(u8),
    InvalidColor(String),
    UnsupportedFeature(&'static str),
    ResponseTimeout,
    ShortWrite { expected: usize, written: usize },
    ResponseTooShort { minimum: usize, received: usize },
    UnexpectedResponse,
    HidppProtocolError(u8),
}
impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported => write!(f, "Device is not supported"),
            Self::InvalidBrightness(v) => write!(f, "Brightness {v} lm is not supported"),
            Self::InvalidTemperature(v) => write!(f, "Temperature {v} K is not supported"),
            Self::InvalidPercentage(v) => write!(
                f,
                "Percentage {v}% is not valid. Only values between 1 and 100 are allowed."
            ),
            Self::HidError(e) => write!(f, "HID error occurred: {e}"),
            Self::UnsupportedDeviceType => write!(f, "Unsupported device type"),
            Self::InvalidZone(v) => {
                write!(f, "Back colour zone {v} is not valid. Only zones 1-7 are allowed.")
            },
            Self::InvalidColor(v) => {
                write!(f, "Back colour {v} is not valid. Only hexadecimal colours are allowed.")
            },
            Self::UnsupportedFeature(v) => {
                write!(f, "Required HID++ feature {v} is unavailable or has not been verified")
            },
            Self::ResponseTimeout => write!(f, "Timed out waiting for a HID++ response"),
            Self::ShortWrite { expected, written } => {
                write!(f, "HID++ write was incomplete: wrote {written} of {expected} bytes")
            },
            Self::ResponseTooShort { minimum, received } => write!(
                f,
                "HID++ response is too short: expected at least {minimum} bytes, got {received}"
            ),
            Self::UnexpectedResponse => {
                write!(f, "Received a HID++ response that does not match the request")
            },
            Self::HidppProtocolError(v) => {
                write!(f, "Device returned HID++ protocol error 0x{v:02x}")
            },
        }
    }
}
impl Error for DeviceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Self::HidError(e) = self {
            Some(e)
        } else {
            None
        }
    }
}
impl From<HidError> for DeviceError {
    fn from(e: HidError) -> Self {
        Self::HidError(e)
    }
}
pub type DeviceResult<T> = Result<T, DeviceError>;

