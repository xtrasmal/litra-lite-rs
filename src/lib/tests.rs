#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    #[derive(Debug, Clone)]
    struct MockTransport(Rc<RefCell<usize>>);

    impl HidTransport for MockTransport {
        fn write(&self, _: &[u8]) -> Result<usize, HidError> {
            *self.0.borrow_mut() += 1;
            Ok(REPORT_SIZE)
        }
        fn read_timeout(&self, _: &mut [u8], _: i32) -> Result<usize, HidError> {
            Ok(0)
        }
    }

    fn beam_lx(mock: MockTransport) -> DeviceHandle {
        DeviceHandle {
            transport: Box::new(mock),
            device_type: DeviceType::LitraBeamLX,
            device_path: String::new(),
            serial_number: None,
            features: Features { illumination: 6, rear_brightness: None, rear_colour: None },
        }
    }

    #[test]
    fn invalid_values_cannot_reach_the_transport() {
        let writes = Rc::new(RefCell::new(0));
        let device = beam_lx(MockTransport(writes.clone()));
        assert!(matches!(
            device.set_brightness_in_lumen(29),
            Err(DeviceError::InvalidBrightness(29))
        ));
        assert!(matches!(
            device.set_brightness_in_lumen(401),
            Err(DeviceError::InvalidBrightness(401))
        ));
        assert!(matches!(
            device.set_temperature_in_kelvin(2600),
            Err(DeviceError::InvalidTemperature(2600))
        ));
        assert!(matches!(
            device.set_temperature_in_kelvin(6600),
            Err(DeviceError::InvalidTemperature(6600))
        ));
        assert!(matches!(
            device.set_temperature_in_kelvin(2750),
            Err(DeviceError::InvalidTemperature(2750))
        ));
        assert!(matches!(
            device.set_back_brightness_percentage(0),
            Err(DeviceError::InvalidPercentage(0))
        ));
        assert!(matches!(
            device.set_back_brightness_percentage(101),
            Err(DeviceError::InvalidPercentage(101))
        ));
        assert!(matches!(device.set_back_color(0, 0, 0, 0), Err(DeviceError::InvalidZone(0))));
        assert!(matches!(device.set_back_color(8, 0, 0, 0), Err(DeviceError::InvalidZone(8))));
        assert_eq!(*writes.borrow(), 0);
    }

    #[test]
    fn unverified_rear_commands_fail_closed_without_writing() {
        let writes = Rc::new(RefCell::new(0));
        let device = beam_lx(MockTransport(writes.clone()));
        assert!(matches!(device.set_back_on(true), Err(DeviceError::UnsupportedFeature(_))));
        assert!(matches!(
            device.set_back_brightness_percentage(50),
            Err(DeviceError::UnsupportedFeature(_))
        ));
        assert!(matches!(
            device.set_back_color(1, 0, 0, 0),
            Err(DeviceError::UnsupportedFeature(_))
        ));
        assert_eq!(*writes.borrow(), 0);
    }

    #[test]
    fn boundaries_and_malformed_responses_are_checked() {
        assert!(Brightness::try_from(30).is_ok());
        assert!(Brightness::try_from(400).is_ok());
        assert!(Temperature::try_from(2700).is_ok());
        assert!(Temperature::try_from(6500).is_ok());
        assert!(matches!(
            ensure_len(&[REPORT_ID, DEVICE_INDEX, 6], 4),
            Err(DeviceError::ResponseTooShort { received: 3, .. })
        ));
    }
}
