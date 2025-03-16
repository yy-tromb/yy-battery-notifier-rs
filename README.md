[!IMPORTANT]
This application is now for windows 10,11 only. This is because of using WinRT API( Windows.Devices.Power.Battery, Windows.System.Power.PowerManager ) and [https://doc.rust-lang.org/beta/rustc/platform-support.html](the Rust's platform support).

# yy-battery-notifier-rs
check battery level and notify you

## Usage
`$ yy-battery-notifier-rs.exe -s "path to settings.toml"`

### Options:
- -s, --settings : path to settings.toml<br>[default: .\default_settings.toml]
- -h, --help : Print help
- -V, --version : Print version
