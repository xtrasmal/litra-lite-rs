    /// Sets the brightness of your Logitech Litra device. By default, all
    /// devices are targeted, unless one or more devices are specified with
    /// --device-type, --serial-number or --device-path.
    #[clap(group = ArgGroup::new("brightness").required(true).multiple(false))]
    Brightness {
        #[clap(
            long,
            short,
            help = SERIAL_NUMBER_ARGUMENT_HELP,
            conflicts_with_all = ["device_path", "device_type"]
        )]
        serial_number: Option<String>,
        #[clap(
            long,
            short('p'),
            help = DEVICE_PATH_ARGUMENT_HELP,
            conflicts_with_all = ["serial_number", "device_type"]
        )]
        device_path: Option<String>,
        #[clap(long, short('t'), help = DEVICE_TYPE_ARGUMENT_HELP, value_parser = DeviceTypeValueParser, conflicts_with_all = ["serial_number", "device_path"])]
        device_type: Option<DeviceType>,
        #[clap(
            long,
            short,
            help = "The brightness to set, measured in lumens. This can be set to any value \
                    between the minimum and maximum for the device returned by the `devices` \
                    command.",
            group = "brightness"
        )]
        value: Option<u16>,
        #[clap(
            long,
            short('b'),
            help = "The brightness to set, as a percentage of the maximum brightness",
            group = "brightness",
            value_parser = clap::value_parser!(u8).range(1..=100)
        )]
        percentage: Option<u8>,
    },
    /// Increases the brightness of your Logitech Litra device. The command will
    /// error if trying to increase the brightness beyond the device's maximum.
    /// By default, all devices are targeted, unless one or more devices are
    /// specified with --device-type, --serial-number or --device-path.
    #[clap(group = ArgGroup::new("brightness-up").required(true).multiple(false))]
    BrightnessUp {
        #[clap(
            long,
            short,
            help = SERIAL_NUMBER_ARGUMENT_HELP,
            conflicts_with_all = ["device_path", "device_type"]
        )]
        serial_number: Option<String>,
        #[clap(
            long,
            short('p'),
            help = DEVICE_PATH_ARGUMENT_HELP,
            conflicts_with_all = ["serial_number", "device_type"]
        )]
        device_path: Option<String>,
        #[clap(long, short('t'), help = DEVICE_TYPE_ARGUMENT_HELP, value_parser = DeviceTypeValueParser, conflicts_with_all = ["serial_number", "device_path"])]
        device_type: Option<DeviceType>,
        #[clap(
            long,
            short,
            help = "The amount to increase the brightness by, measured in lumens.",
            group = "brightness-up"
        )]
        value: Option<u16>,
        #[clap(
            long,
            short,
            help = "The number of percentage points to increase the brightness by",
            group = "brightness-up",
            value_parser = clap::value_parser!(u8).range(1..=100)
        )]
        percentage: Option<u8>,
    },
    /// Decreases the brightness of your Logitech Litra device. The command will
    /// error if trying to decrease the brightness below the device's minimum.
    /// By default, all devices are targeted, unless one or more devices are
    /// specified with --device-type, --serial-number or --device-path.
    #[clap(group = ArgGroup::new("brightness-down").required(true).multiple(false))]
    BrightnessDown {
        #[clap(
            long,
            short,
            help = SERIAL_NUMBER_ARGUMENT_HELP,
            conflicts_with_all = ["device_path", "device_type"]
        )]
        serial_number: Option<String>,
        #[clap(
            long,
            short('p'),
            help = DEVICE_PATH_ARGUMENT_HELP,
            conflicts_with_all = ["serial_number", "device_type"]
        )]
        device_path: Option<String>,
        #[clap(long, short('t'), help = DEVICE_TYPE_ARGUMENT_HELP, value_parser = DeviceTypeValueParser, conflicts_with_all = ["serial_number", "device_path"])]
        device_type: Option<DeviceType>,
        #[clap(
            long,
            short,
            help = "The amount to decrease the brightness by, measured in lumens.",
            group = "brightness-down"
        )]
        value: Option<u16>,
        #[clap(
            long,
            short,
            help = "The number of percentage points to reduce the brightness by",
            group = "brightness-down",
            value_parser = clap::value_parser!(u8).range(1..=100)
        )]
        percentage: Option<u8>,
    },
