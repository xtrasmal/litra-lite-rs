trait HidTransport: fmt::Debug {
    fn write(&self, p: &[u8]) -> Result<usize, HidError>;
    fn read_timeout(&self, b: &mut [u8], ms: i32) -> Result<usize, HidError>;
}
#[derive(Debug)]
struct HidDeviceTransport(HidDevice);
impl HidTransport for HidDeviceTransport {
    fn write(&self, p: &[u8]) -> Result<usize, HidError> {
        self.0.write(p)
    }
    fn read_timeout(&self, b: &mut [u8], ms: i32) -> Result<usize, HidError> {
        self.0.read_timeout(b, ms)
    }
}
#[derive(Debug, Clone, Copy)]
struct Features {
    illumination: u8,
    rear_brightness: Option<u8>,
    rear_colour: Option<u8>,
}
