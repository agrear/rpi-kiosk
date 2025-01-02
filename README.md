# rpi-kiosk

Minimal app for displaying a webpage on a Raspberry Pi.


## Prerequisites

You will need a Raspberry Pi with ARMv8 architecture running Debian 12 (Bookworm) 64bit. Although it might also work on Debian 11 (Bullseye) 64bit.

This includes:
- Raspberry Pi 3
- Raspberry Pi 4
- Raspberry Pi 5
- Raspberry Pi Zero 2 W


## Installation

1. Download a .deb package from the [Releases](https://github.com/agrear/rpi-kiosk/releases).
2. Run e.g. `dpkg -i rpi-kiosk_1.0.0_arm64.deb` to install it.
3. (Optional) Add an autostart entry; e.g. for labwc (Lab Wayland Compositor):
    - `nano ~/.config/labwc/autostart`
    - `/usr/bin/rpi-kiosk https://www.example.com/`


## Usage

The following command line arguments are supported:
- `--hide-cursor` : Hide the mouse cursor
- `--url` : URL of the webpage to display

Note: If you don't pass the `--url` argument the first rest argument is used for the URL.

### Examples

- /usr/bin/rpi-kiosk https://www.example.com/
- /usr/bin/rpi-kiosk --url "https://www.example.com/" --hide-cursor


## Building from Source

If you want to build the .deb package from source follow the instructions at [Cross-Compiling Tauri Applications for ARM-based Devices](https://v1.tauri.app/v1/guides/building/linux/#cross-compiling-tauri-applications-for-arm-based-devices).


## Troubleshooting

P: When I launch `rpi-kiosk` I get an error message saying that `libwebkit2gtk` cannot be found.

S: Install [Libwebkit2gtk-4.0-37](https://pkgs.org/download/libwebkit2gtk-4.0-37) by running `sudo apt install libwebkit2gtk-4.0-37`
