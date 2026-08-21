#[cfg(feature = "cli")]
#[derive(Debug, clap::Args)]
struct Target {
    #[clap(long, short, help = SERIAL_NUMBER_ARGUMENT_HELP, conflicts_with_all = ["device_path", "device_type"])]
    serial_number: Option<String>,
    #[clap(long, short('p'), help = DEVICE_PATH_ARGUMENT_HELP, conflicts_with_all = ["serial_number", "device_type"])]
    device_path: Option<String>,
    #[clap(long, short('t'), help = DEVICE_TYPE_ARGUMENT_HELP, value_parser = DeviceTypeValueParser, conflicts_with_all = ["serial_number", "device_path"])]
    device_type: Option<DeviceType>,
}

#[cfg(feature = "cli")]
#[derive(Debug, clap::Args)]
struct BackTarget {
    #[clap(long, short, help = SERIAL_NUMBER_ARGUMENT_HELP, conflicts_with = "device_path")]
    serial_number: Option<String>,
    #[clap(long, short('p'), help = DEVICE_PATH_ARGUMENT_HELP, conflicts_with = "serial_number")]
    device_path: Option<String>,
}

#[cfg(feature = "cli")]
#[derive(Debug, clap::Args)]
#[command(group = ArgGroup::new("brightness").required(true).multiple(false))]
struct BrightnessAmount {
    #[clap(long, short, group = "brightness")]
    value: Option<u16>,
    #[clap(long, short('b'), group = "brightness", value_parser = clap::value_parser!(u8).range(1..=100))]
    percentage: Option<u8>,
}

#[cfg(feature = "cli")]
#[derive(Debug, clap::Args)]
struct BrightnessArgs {
    #[command(flatten)]
    target: Target,
    #[command(flatten)]
    amount: BrightnessAmount,
}

#[cfg(feature = "cli")]
#[derive(Debug, clap::Args)]
struct TemperatureArgs {
    #[command(flatten)]
    target: Target,
    #[clap(long, short)]
    value: u16,
}

#[cfg(feature = "cli")]
#[derive(Debug, clap::Args)]
struct BackColorArgs {
    #[command(flatten)]
    target: BackTarget,
    #[clap(long, short, conflicts_with = "color")]
    value: Option<String>,
    #[clap(long, short, value_enum, conflicts_with = "value")]
    color: Option<NamedColor>,
    #[clap(long, short, value_parser = clap::value_parser!(u8).range(1..=7))]
    zone: Option<u8>,
}

#[cfg(feature = "cli")]
#[derive(Debug, clap::Args)]
struct BackBrightnessArgs {
    #[command(flatten)]
    target: BackTarget,
    #[clap(long, short, value_parser = clap::value_parser!(u8).range(1..=100))]
    percentage: u8,
}

#[cfg(feature = "cli")]
#[derive(Debug, Subcommand)]
enum Commands {
    On(Target),
    Off(Target),
    Toggle(Target),
    Brightness(BrightnessArgs),
    BrightnessUp(BrightnessArgs),
    BrightnessDown(BrightnessArgs),
    Temperature(TemperatureArgs),
    TemperatureUp(TemperatureArgs),
    TemperatureDown(TemperatureArgs),
    BackColor(BackColorArgs),
    BackBrightness(BackBrightnessArgs),
    BackOff(BackTarget),
    BackOn(BackTarget),
    BackToggle(BackTarget),
    BackBrightnessUp(BackBrightnessArgs),
    BackBrightnessDown(BackBrightnessArgs),
    Devices { #[clap(long, short, action)] json: bool },
}
