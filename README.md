> [!IMPORTANT]
> This application is now for ***windows 10,11 only***. This is because of using WinRT API( Windows.Devices.Power.Battery, Windows.System.Power.PowerManager ) and [the Rust's platform support](https://doc.rust-lang.org/beta/rustc/platform-support.html).  
  
> [!IMPORTANT]
> The notification API of WinRT require Application User Model ID. If you install with WIX installer, Application User Model ID is set in Windows Registry and This app can toast.
# yy-battery-notifier-rs
check battery level and notify you

## Usage
`yy-battery-notifier-rs.exe -s "path to settings.toml"`

### Commands:
  registry - register,delete  
  startup - register,delete  

### Options:
  -s, --settings <path to settings.toml>  \[default: .\settings.toml]  
  -d, --default_settings : Use [default_settings.toml](https://github.com/yy-tromb/yy-battery-notifier-rs/blob/main/default_settings.toml)  
      --msgbox : When error occurs, let you know by messagebox  
  -h, --help : Print help  
  -V, --version : Print version  
