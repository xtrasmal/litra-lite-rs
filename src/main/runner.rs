#[cfg(feature = "cli")]
type BrightnessHandler =
    fn(Option<&str>, Option<&str>, Option<&DeviceType>, Option<u16>, Option<u8>) -> CliResult;
#[cfg(feature = "cli")]
type TemperatureHandler = fn(Option<&str>, Option<&str>, Option<&DeviceType>, u16) -> CliResult;

#[cfg(feature = "cli")]
fn main() -> ExitCode {
    if let Some(latest_version) = check_for_updates() {
        eprintln!("{}", format_update_message(&latest_version));
    }

    let result = match &Cli::parse().command {
        Commands::Devices { json } => handle_devices_command(*json),
        Commands::On(args) => handle_on_command(args.serial_number.as_deref(), args.device_path.as_deref(), args.device_type.as_ref()),
        Commands::Off(args) => handle_off_command(args.serial_number.as_deref(), args.device_path.as_deref(), args.device_type.as_ref()),
        Commands::Toggle(args) => handle_toggle_command(args.serial_number.as_deref(), args.device_path.as_deref(), args.device_type.as_ref()),
        Commands::Brightness(args) => brightness_command(args, handle_brightness_command),
        Commands::BrightnessUp(args) => brightness_command(args, handle_brightness_up_command),
        Commands::BrightnessDown(args) => brightness_command(args, handle_brightness_down_command),
        Commands::Temperature(args) => temperature_command(args, handle_temperature_command),
        Commands::TemperatureUp(args) => temperature_command(args, handle_temperature_up_command),
        Commands::TemperatureDown(args) => temperature_command(args, handle_temperature_down_command),
        Commands::BackColor(args) => {
            let hex = match (&args.value, args.color) {
                (Some(value), None) => value.clone(),
                (None, Some(color)) => color.to_hex().to_owned(),
                _ => unreachable!("clap requires exactly one colour"),
            };
            handle_back_color_command(args.target.serial_number.as_deref(), args.target.device_path.as_deref(), &hex, args.zone)
        }
        Commands::BackBrightness(args) => handle_back_brightness_command(args.target.serial_number.as_deref(), args.target.device_path.as_deref(), args.percentage),
        Commands::BackOff(args) => handle_back_off_command(args.serial_number.as_deref(), args.device_path.as_deref()),
        Commands::BackOn(args) => handle_back_on_command(args.serial_number.as_deref(), args.device_path.as_deref()),
        Commands::BackToggle(args) => handle_back_toggle_command(args.serial_number.as_deref(), args.device_path.as_deref()),
        Commands::BackBrightnessUp(args) => handle_back_brightness_up_command(args.target.serial_number.as_deref(), args.target.device_path.as_deref(), args.percentage),
        Commands::BackBrightnessDown(args) => handle_back_brightness_down_command(args.target.serial_number.as_deref(), args.target.device_path.as_deref(), args.percentage),
    };
    result.map_or_else(
        |error| {
            eprintln!("{error}");
            ExitCode::FAILURE
        },
        |_| ExitCode::SUCCESS,
    )
}

fn brightness_command(
    args: &BrightnessArgs,
    command: BrightnessHandler,
) -> CliResult {
    command(args.target.serial_number.as_deref(), args.target.device_path.as_deref(), args.target.device_type.as_ref(), args.amount.value, args.amount.percentage)
}

fn temperature_command(
    args: &TemperatureArgs,
    command: TemperatureHandler,
) -> CliResult {
    command(args.target.serial_number.as_deref(), args.target.device_path.as_deref(), args.target.device_type.as_ref(), args.value)
}
