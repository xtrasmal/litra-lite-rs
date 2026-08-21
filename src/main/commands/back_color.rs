    /// Set the color of one or more zones on the back of your Logitech Litra
    /// Beam LX device. By default, all Litra Beam LX devices are targeted,
    /// unless a specific device is specified with --serial-number or
    /// --device-path.
    #[clap(group = ArgGroup::new("color-input").required(true).multiple(false))]
    BackColor {
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
            short,
            help = "The hexadecimal color code to use (e.g. FF0000 for red). Either --value or \
                    --color must be specified.",
            group = "color-input"
        )]
        value: Option<String>,
        #[clap(
            long,
            short,
            help = "A named color to use. Either --value or --color must be specified.",
            group = "color-input"
        )]
        color: Option<NamedColor>,
        #[clap(
            long,
            short('z'),
            help = "The zone of the light to control, numbered 1 to 7 from left to right. If not \
                    specified, all zones will be targeted."
        )]
        zone: Option<u8>,
    },
