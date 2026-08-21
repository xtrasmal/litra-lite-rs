use std::fmt;
use std::process::ExitCode;
use std::str::FromStr;
use std::time::Duration;

use clap::builder::TypedValueParser;
use clap::{ArgGroup, Parser, Subcommand, ValueEnum};
#[cfg(feature = "cli")]
use colored::Colorize;
use litra::{Device, DeviceError, DeviceHandle, DeviceResult, DeviceType, Litra};
use serde::Serialize;
#[cfg(feature = "cli")]
use tabled::{Table, Tabled};

// Custom parser for DeviceType
#[derive(Debug, Clone)]
struct DeviceTypeValueParser;

impl TypedValueParser for DeviceTypeValueParser {
    type Value = DeviceType;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let value_str = value.to_string_lossy();
        DeviceType::from_str(&value_str).map_err(|_| {
            let mut err = clap::Error::new(clap::error::ErrorKind::InvalidValue);
            if let Some(arg) = arg {
                err.insert(
                    clap::error::ContextKind::InvalidArg,
                    clap::error::ContextValue::String(arg.to_string()),
                );
            }
            err.insert(
                clap::error::ContextKind::Custom,
                clap::error::ContextValue::String(format!("Invalid device type: {}", value_str)),
            );
            err
        })
    }
}

/// Control your USB-connected Logitech Litra lights from the command line
#[cfg(feature = "cli")]
#[derive(Debug, Parser)]
#[clap(
    name = "litra",
    version,
    after_long_help = "This CLI automatically checks for updates once per day. To disable update \
                       checks, set the LITRA_DISABLE_UPDATE_CHECK environment variable to any \
                       value."
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

const SERIAL_NUMBER_ARGUMENT_HELP: &str = "Specify the device to target by its serial number";
const DEVICE_PATH_ARGUMENT_HELP: &str =
    "Specify the device to target by its path (useful for devices that don't show a serial number)";
const DEVICE_TYPE_ARGUMENT_HELP: &str =
    "Specify the device to target by its type (`glow`, `beam` or `beam_lx`)";

/// Named colors for the back-color command
#[cfg(feature = "cli")]
#[derive(Debug, Clone, Copy, ValueEnum)]
enum NamedColor {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
    Pink,
    Cyan,
    White,
    Magenta,
}

#[cfg(feature = "cli")]
impl NamedColor {
    fn to_hex(self) -> &'static str {
        match self {
            NamedColor::Red => "FF0000",
            NamedColor::Green => "00FF00",
            NamedColor::Blue => "0000FF",
            NamedColor::Yellow => "FFFF00",
            NamedColor::Orange => "FFA500",
            NamedColor::Purple => "800080",
            NamedColor::Pink => "FFC0CB",
            NamedColor::Cyan => "00FFFF",
            NamedColor::White => "FFFFFF",
            NamedColor::Magenta => "FF00FF",
        }
    }
}

