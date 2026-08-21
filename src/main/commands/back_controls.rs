    /// Set the brightness of the colorful backlight on your Logitech Litra Beam
    /// LX device. By default, all Litra Beam LX devices are targeted, unless a
    /// specific device is specified with --serial-number or --device-path.
    BackBrightness {
        #[clap(
            long,
            short,
            help = SERIAL_NUMBER_ARGUMENT_HELP,
            conflicts_with = "device_path"
        )]
        serial_number: Option<String>,
        #[clap(
            long,
            short('p'),
            help = DEVICE_PATH_ARGUMENT_HELP,
            conflicts_with = "serial_number"
        )]
        device_path: Option<String>,
        #[clap(
            long,
            short('b'),
            help = "The brightness to set, as a percentage of the maximum brightness",
            value_parser = clap::value_parser!(u8).range(1..=100)
        )]
        percentage: u8,
    },
    /// Turn off the colorful backlight on your Logitech Litra Beam LX device.
    /// By default, all Litra Beam LX devices are targeted, unless a specific
    /// device is specified with --serial-number or --device-path.
    BackOff {
        #[clap(
            long,
            short,
            help = SERIAL_NUMBER_ARGUMENT_HELP,
            conflicts_with = "device_path"
        )]
        serial_number: Option<String>,
        #[clap(
            long,
            short('p'),
            help = DEVICE_PATH_ARGUMENT_HELP,
            conflicts_with = "serial_number"
        )]
        device_path: Option<String>,
    },
    /// Turn on the colorful backlight on your Logitech Litra Beam LX device. By
    /// default, all Litra Beam LX devices are targeted, unless a specific
    /// device is specified with --serial-number or --device-path.
    BackOn {
        #[clap(
            long,
            short,
            help = SERIAL_NUMBER_ARGUMENT_HELP,
            conflicts_with = "device_path"
        )]
        serial_number: Option<String>,
        #[clap(
            long,
            short('p'),
            help = DEVICE_PATH_ARGUMENT_HELP,
            conflicts_with = "serial_number"
        )]
        device_path: Option<String>,
    },
    /// Toggles the colorful backlight on your Logitech Litra Beam LX device on
    /// or off. By default, all Litra Beam LX devices are targeted, unless a
    /// specific device is specified with --serial-number or --device-path.
    BackToggle {
        #[clap(
            long,
            short,
            help = SERIAL_NUMBER_ARGUMENT_HELP,
            conflicts_with = "device_path"
        )]
        serial_number: Option<String>,
        #[clap(
            long,
            short('p'),
            help = DEVICE_PATH_ARGUMENT_HELP,
            conflicts_with = "serial_number"
        )]
        device_path: Option<String>,
    },
    /// Increases the brightness of the colorful backlight on your Logitech
    /// Litra Beam LX device. The command will error if trying to increase the
    /// brightness beyond 100%. By default, all Litra Beam LX devices are
    /// targeted, unless a specific device is specified with --serial-number or
    /// --device-path.
    BackBrightnessUp {
        #[clap(
            long,
            short,
            help = SERIAL_NUMBER_ARGUMENT_HELP,
            conflicts_with = "device_path"
        )]
        serial_number: Option<String>,
        #[clap(
            long,
            short('p'),
            help = DEVICE_PATH_ARGUMENT_HELP,
            conflicts_with = "serial_number"
        )]
        device_path: Option<String>,
        #[clap(
            long,
            short('b'),
            help = "The number of percentage points to increase the brightness by",
            value_parser = clap::value_parser!(u8).range(1..=100)
        )]
        percentage: u8,
    },
    /// Decreases the brightness of the colorful backlight on your Logitech
    /// Litra Beam LX device. The command will error if trying to decrease the
    /// brightness below 1%. By default, all Litra Beam LX devices are targeted,
    /// unless a specific device is specified with --serial-number or
    /// --device-path.
    BackBrightnessDown {
        #[clap(
            long,
            short,
            help = SERIAL_NUMBER_ARGUMENT_HELP,
            conflicts_with = "device_path"
        )]
        serial_number: Option<String>,
        #[clap(
            long,
            short('p'),
            help = DEVICE_PATH_ARGUMENT_HELP,
            conflicts_with = "serial_number"
        )]
        device_path: Option<String>,
        #[clap(
            long,
            short('b'),
            help = "The number of percentage points to decrease the brightness by",
            value_parser = clap::value_parser!(u8).range(1..=100)
        )]
        percentage: u8,
    },
