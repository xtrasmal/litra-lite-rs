    /// Turn your Logitech Litra device on. By default, all devices are
    /// targeted, unless one or more devices are specified with --device-type,
    /// --serial-number or --device-path.
    On {
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
    },
    /// Turn your Logitech Litra device off. By default, all devices are
    /// targeted, unless one or more devices are specified with --device-type,
    /// --serial-number or --device-path.
    Off {
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
    },
    /// Toggles your Logitech Litra device on or off. By default, all devices
    /// are targeted, unless one or more devices are specified with
    /// --device-type, --serial-number or --device-path.
    Toggle {
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
    },
