enum Command {
    GetPower,
    SetPower(Power),
    GetBrightness,
    SetBrightness(Brightness),
    GetTemperature,
    SetTemperature(Temperature),
    GetRearBrightness,
    SetRearBrightness(BackBrightness),
    GetRearPower,
    SetRearPower(Power),
}
impl Command {
    fn packet(self, feature: u8) -> [u8; REPORT_SIZE] {
        let (fun, p) = match self {
            Self::GetPower => (0x01, [0; 16]),
            Self::SetPower(Power(v)) => {
                (0x1c, [u8::from(v), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
            },
            Self::GetBrightness => (0x31, [0; 16]),
            Self::SetBrightness(Brightness(v)) => {
                let [a, b] = v.to_be_bytes();
                (0x4c, [a, b, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
            },
            Self::GetTemperature => (0x81, [0; 16]),
            Self::SetTemperature(Temperature(v)) => {
                let [a, b] = v.to_be_bytes();
                (0x9c, [a, b, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
            },
            Self::GetRearBrightness => (0x1c, [0; 16]),
            Self::SetRearBrightness(BackBrightness(v)) => {
                (0x2c, [0, v, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
            },
            Self::GetRearPower => (0x3c, [0; 16]),
            Self::SetRearPower(Power(v)) => {
                (0x4c, [u8::from(v), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
            },
        };
        let mut r = [0; REPORT_SIZE];
        r[..4].copy_from_slice(&[REPORT_ID, DEVICE_INDEX, feature, fun]);
        r[4..].copy_from_slice(&p);
        r
    }
}
fn root_packet(feature: u16) -> [u8; REPORT_SIZE] {
    let [a, b] = feature.to_be_bytes();
    let mut p = [0; REPORT_SIZE];
    p[..6].copy_from_slice(&[REPORT_ID, DEVICE_INDEX, 0, SOFTWARE_ID, a, b]);
    p
}
fn resolve_feature(transport: &dyn HidTransport, feature: u16) -> DeviceResult<u8> {
    let response = request(transport, root_packet(feature))?;
    ensure_len(&response, 5)?;
    Ok(response[4])
}
fn write_known(transport: &dyn HidTransport, packet: &[u8]) -> DeviceResult<()> {
    let written = transport.write(packet)?;
    if written == packet.len() {
        Ok(())
    } else {
        Err(DeviceError::ShortWrite { expected: packet.len(), written })
    }
}
fn request(t: &dyn HidTransport, p: [u8; REPORT_SIZE]) -> DeviceResult<[u8; REPORT_SIZE]> {
    let written = t.write(&p)?;
    if written != p.len() {
        return Err(DeviceError::ShortWrite { expected: p.len(), written });
    }
    let mut r = [0; REPORT_SIZE];
    let n = t.read_timeout(&mut r, READ_TIMEOUT_MS)?;
    if n == 0 {
        return Err(DeviceError::ResponseTimeout);
    }
    ensure_len(&r[..n], 4)?;
    if r[2] == ERROR_FEATURE {
        ensure_len(&r[..n], 6)?;
        if r[3] != p[2] || r[4] != p[3] {
            return Err(DeviceError::UnexpectedResponse);
        }
        return Err(DeviceError::HidppProtocolError(r[5]));
    }
    if r[..4] != p[..4] {
        return Err(DeviceError::UnexpectedResponse);
    }
    Ok(r)
}
fn ensure_len(r: &[u8], n: usize) -> DeviceResult<()> {
    if r.len() < n {
        Err(DeviceError::ResponseTooShort { minimum: n, received: r.len() })
    } else {
        Ok(())
    }
}
fn device_type_from_product_id(id: u16) -> Option<DeviceType> {
    match id {
        0xc900 => Some(DeviceType::LitraGlow),
        0xc901 | 0xb901 => Some(DeviceType::LitraBeam),
        0xc903 | 0xb903 => Some(DeviceType::LitraBeamLX),
        _ => None,
    }
}

