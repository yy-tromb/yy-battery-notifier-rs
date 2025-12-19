> [!IMPORTANT]
> This application is now for ***windows 10,11 only***. This is because of using WinRT API( Windows.Devices.Power.Battery, Windows.System.Power.PowerManager ) and [the Rust's platform support](https://doc.rust-lang.org/beta/rustc/platform-support.html).

> [!IMPORTANT]
> The notification API of WinRT require Application User Model ID. If you install with WIX installer, Application User Model ID is set in Windows Registry and This app can toast.

# yy-battery-notifier-rs

check battery level and notify you

## Installation

You can install [from here](https://github.com/yy-tromb/yy-battery-notifier-rs/releases).
I recommend MSI installer.

## About settings.toml

About TOML format, try google this...
default_settings.toml:

```default_settings.toml
check_interval = 60
# interval seconds(integer) for check battery level.

[[notifications]]
percentage = "90+"
# express condition when notifications should be started.
# This consists of percentage(integer) and suffix "+" or "-".
# This example express that notifications will start whether battery level excess 90%

power_supply = "Adequate"
# express power supply status. This consists of "Adequate", "InAdequate", "None".

title = "Remove the plug!"
# notification title

message = "Your PC is now fully charged. Remove the plug"
# notification message

[[notifications]]
percentage = "45-"
power_supply = "None"
title = "Plug in!"
message = "The battery level of Your PC is low. Plug in."
```

## Usage on GUI

## Usage on CLI

### Start notify with specified settings.toml

`yy-battery-notifier-rs.exe -s "path to settings.toml"`

### Start notify with default settings

`yy-battery-notifier-rs.exe -d`

### Commands

  registry - register,delete : register or delete Application User Model Id (used to notify as toast) on Windows Registry
  startup - register,delete : register or delete this app as startup app. Even if you register twice or more, settings will be overrided.

### Options

  -s, --settings "path to settings.toml"  \[default: .\settings.toml]
  -d, --default_settings : Use [default_settings.toml](https://github.com/yy-tromb/yy-battery-notifier-rs/blob/main/default_settings.toml)
      --msgbox : When error occurs, let you know by messagebox
  -h, --help : Print help
  -V, --version : Print version

## ToDos
- Add mode change button
- Show mode selector button when this program starts
- Implement my notification crate ( that can show both progress bar and input )
