# litra-lite — instructions

Control a Logitech Litra Beam LX (front white key light + RGB back light) over
Bluetooth from the command line. No Logitech software required.

## Requirements

- macOS with a Bluetooth adapter
- A Rust toolchain (`cargo`)
- A Logitech Litra Beam LX

## One-time setup: bond the light over Bluetooth

1. Turn the light on.
2. Open **System Settings > Bluetooth** (the macOS pane, not any Logitech app).
3. Pair **Litra Beam LX**.

macOS bridges the bonded light into its HID subsystem, so the tool then drives it
the same way it drives a USB-connected Litra. Control stays available while the
light remains bonded; choosing **Forget This Device** removes it and the tool will
no longer see it.

## Build

Install the `litra` binary:

```sh
cargo install --path .
```

Or build in place and run from the target directory:

```sh
cargo build --release
./target/release/litra <command>
```

## Verify the light is seen

```sh
litra devices          # human-readable table
litra devices --json   # machine-readable
```

The row reports both lights: front status/brightness/temperature and back
status/brightness.

## Front light (white key light)

```sh
litra on
litra off
litra toggle
litra brightness --percentage 80      # 1-100%
litra brightness --value 400          # absolute lumens
litra brightness-up --percentage 10
litra brightness-down --percentage 10
litra temperature --value 4500        # Kelvin, multiple of 100
litra temperature-up --value 100
litra temperature-down --value 100
```

Use `litra devices` to read a light's valid lumen and Kelvin ranges.

## Back light (RGB)

```sh
litra back-on
litra back-off
litra back-toggle
litra back-brightness --percentage 30       # 1-100%
litra back-brightness-up --percentage 10
litra back-brightness-down --percentage 10
litra back-color --color red                # named color
litra back-color --value FF0000             # hex color
litra back-color --value 00FF00 --zone 3    # zones 1-7, left to right
```

Omit `--zone` to set every zone at once.

## Targeting a specific device

Commands act on all matching Litra devices by default. Narrow to one with:

- `--serial-number <serial>`
- `--device-path <path>` (use when a device reports no serial number)
- `--device-type glow|beam|beam_lx`

Over Bluetooth the Beam LX reports no serial number, so target it by
`--device-path` (take the value from `litra devices`) when more than one Litra is
connected.

## Update check

The CLI checks GitHub for a newer release once per day. To disable it, set:

```sh
export LITRA_DISABLE_UPDATE_CHECK=1
```
