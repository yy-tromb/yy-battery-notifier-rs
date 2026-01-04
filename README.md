Languages: English | [日本語(Japanese)](https://github.com/yy-tromb/yy-battery-notifier-rs/blob/main/docs/ja/README.md)

# yy-battery-notifier-rs
check battery level and notify you

> [!IMPORTANT]
> This application is now for ***windows 10,11 only***. This is because of using WinRT API( Windows.Devices.Power.Battery, Windows.System.Power.PowerManager ) and [the Rust's platform support](https://doc.rust-lang.org/beta/rustc/platform-support.html).

> [!IMPORTANT]
> The notification API of WinRT require Application User Model ID. If you install with MSI installer, Application User Model ID is set in Windows Registry and This app can toast.

## Installation

You can install [GitHub Releases](https://github.com/yy-tromb/yy-battery-notifier-rs/releases).
I recommend MSI installer.  
Or, you can build with [Cargo](https://doc.rust-lang.org/cargo/) as below...  
normal release build: `cargo build --release` or `cargo b -r`  
with gui feature: `cargo build --release --features gui` or `cargo b -r --features gui`

### MSI Installer
You can find from [GitHub Releases](https://github.com/yy-tromb/yy-battery-notifier-rs/releases).  
The default install location is `%ProgramFiles%\yy-tromb\yy-battery-notifier-rs\`.  
You can uninstall from Control Panel or Windows Settings.  
The installer also create shortcuts on start menu as below...
![Screenshot of part of list of apps in start menu](docs/assets/shortcuts_msi_installed.png)
- Delete Startup: delete startup settings from Windows registry.
- Register Startup: register startup settings to Windows registry with user specified settings (as input).
- Register Startup with Default Settings: register startup settings to Windows registry with default settings with [default_settings.toml](https://github.com/yy-tromb/yy-battery-notifier-rs/blob/main/default_settings.toml).

</details>

## Usage on notification
### method: "TauriWinrtToast"
#### notify battery level
<img alt="screenshot of notification of battery level" style="width: 60%" src="docs/assets/tauri_battery_notify.png">

#### notification of mode changing
<img alt="screenshot of notification of mode changing" style="width: 60%" src="docs/assets/tauri_mode_change_notify.png">

### method: "WinrtToastReborn"
#### notify battery level (input_type = "ModeSelector")
<img alt="screenshot of notification of battery level with settings that input_type defines as mode selector" style="width: 60%" src="docs/assets/reborn_battery_notify_selector.png">

#### notify battery level (input_type = "SilentSpecifiedMinutes")
<img alt="screenshot of notification of battery level with settings that input_type defines as silent specified minutes" style="width: 60%" src="docs/assets/reborn_battery_notify_specified.png">

#### notification of mode changing
<img alt="screenshot of notification of mode changing" style="width: 60%" src="docs/assets/reborn_mode_change_notify.png">
<img alt="screenshot of notification of mode changing with expand selector" style="width: 60%" src="docs/assets/reborn_mode_change_notify_expand-selector.png">

## Usage on CLI
### Start notify with specified settings.toml
`yy-battery-notifier-rs.exe -s "path to settings.toml"`

### Start notify with default settings ([this](https://github.com/yy-tromb/yy-battery-notifier-rs/blob/main/default_settings.toml))
`yy-battery-notifier-rs.exe -d`

### Sub commands
  aumid - register,delete : register or delete Application User Model Id (used to notify as toast) on Windows Registry  
  startup - register,delete : register or delete this app as startup app. Even if you register twice or more, settings will be overrided.

### Options

  -s, --settings "path to settings.toml"  \[default: .\settings.toml]  
  -d, --default_settings : Use [default_settings.toml](https://github.com/yy-tromb/yy-battery-notifier-rs/blob/main/default_settings.toml)  
      --msgbox : When error occurs, let you know by messagebox  
  -h, --help : Print help  
  -V, --version : Print version  

## About settings.toml

About TOML format, try google this...

> [!NOTE]
> "TauriWinrtToast" method does not implement input element, so "input_type" field of each notification setting is not effective.

### [default_settings.toml](https://github.com/yy-tromb/yy-battery-notifier-rs/blob/main/default_settings.toml)

```default_settings.toml
check_interval = 60
# interval seconds(integer) for check battery level.

# ==================================================
# Optional fields below:

notification_method = "TauriWinrtToast"
# Optional field
# This field defines notification method (notifier crate).
# Options now: "TauriWinrtToast"(default), "WinrtToastReborn"

mode_names = [""]
# Optional field
# This field defines order of mode.
# The order of modes written in file is mostly different because of using hash table.
# This field exists to set the order of modes as you defined.
# The modes not written in this field will be added to position later than modes written in this field.
# The modes not defined `notifications` will not be added to modes list.

initial_mode = ""
# Optional field
# This field defines mode to use initially.
# The default is modeless ("").

abort_on_error_except_initialize = false
# Optional field
# This field defines whether to abort on error except initialize.
# Options: true, false(default)

notify_battery_during_change_mode = false
# Optional field
# This field defines whether to notify battery level during notify change mode.
# Options: true, false(default)

select_mode_when_starts = true
# Optional field
# This field defines whether to select mode when starts.
# Options: true(default), false

wait_seconds_after_select_mode_notify_when_starts = 10
# Optional field
# This field defines seconds to wait after select mode when starts.
# The default is 10 seconds.


# Optional fields end
# ==================================================

[[notifications]]
# `notifications` is optional, but each field (not optional) of a notification setting is required.
# This fields define notifications that always notify regardless of mode.

percentage = "90+"
# define condition when notifications should be started.
# This consists of percentage(integer) and suffix "+" or "-".
# This example express that notifications will start whether battery level excess 90%

power_supply = "Adequate"
# define power supply status.
# This consists of "Adequate", "InAdequate", "None".

title = "Remove the plug!"
# define notification title

message = "Your PC is now fully charged. Remove the plug"
# define notification message

input_type = "SilentSpecifiedMinutes"
# Optional field
# This field defines type of `input` element of notification.
# Options: "ModeSelector"(default), "SilentSpecifiedMinutes"
# "TauriWinrtToast" method does not implement input element, so this field is ignored.

[[notifications]]
percentage = "45-"
power_supply = "None"
title = "Plug in!"
message = "The battery level of Your PC is low. Plug in."
```

### settings with modes
> [!IMPORTANT]
> `modes` is available from version 0.4.0.

```settings_mode_partial.toml
check_interval = 60
# interval seconds(integer) for check battery level.

mode_names = ["default","RetainCharged","RetainMoreCharged"]
# Optional field
# This field defines order of mode.
# The order of modes written in file is mostly different because of using hash table.
# This field exists to set the order of modes as you defined.
# The modes not written in this field will be added to position later than modes written in this field.
# The modes not defined `notifications` will not be added to modes list.

initial_mode = "default"
# Optional field
# This field defines mode to use initially.
# The default is modeless ("").

# =======================================================

[[modes.default.notifications]]
# Name of mode is free, but you must follow the syntax rules of TOML.
# This fields of example define notifications that notify when mode is "default".

percentage = "92+"
# define condition when notifications should be started.
# This consists of percentage(integer) and suffix "+" or "-".
# This example express that notifications will start whether battery level excess 90%

power_supply = "Adequate"
# define power supply status.
# This consists of "Adequate", "InAdequate", "None".

title = "Remove the plug!"
# define notification title

message = "Your PC is now fully charged. Remove the plug."
# define notification message

input_type = "ModeSelector"
# Optional field
# This field defines type of `input` element of notification.
# Options: "ModeSelector"(default), "SilentSpecifiedMinutes"
# "TauriWinrtToast" method does not implement input element, so this field is ignored.
```

<details>
<summary>see full version</summary>

```settings_mode.toml
check_interval = 60
# interval seconds(integer) for check battery level.

notification_method = "TauriWinrtToast"
# Optional field
# This field defines notification method (notifier crate).
# Options now: "TauriWinrtToast"(default), "WinrtToastReborn"

mode_names = ["default","RetainCharged","RetainMoreCharged"]
# Optional field
# This field defines order of mode.
# The order of modes written in file is mostly different because of using hash table.
# This field exists to set the order of modes as you defined.
# The modes not written in this field will be added to position later than modes written in this field.
# The modes not defined `notifications` will not be added to modes list.

initial_mode = "default"
# Optional field
# This field defines mode to use initially.
# The default is modeless ("").

# =======================================================

[[modes.default.notifications]]
# Name of mode is free, but you must follow the rules of TOML.
# This fields define notifications that notify when mode is "default".

percentage = "92+"
# define condition when notifications should be started.
# This consists of percentage(integer) and suffix "+" or "-".
# This example express that notifications will start whether battery level excess 90%

power_supply = "Adequate"
# define power supply status.
# This consists of "Adequate", "InAdequate", "None".

title = "Remove the plug!"
# define notification title

message = "Your PC is now fully charged. Remove the plug."
# define notification message

input_type = "ModeSelector"
# Optional field
# This field defines type of `input` element of notification.
# Options: "ModeSelector"(default), "SilentSpecifiedMinutes"
# "TauriWinrtToast" method does not implement input element, so this field is ignored.

[[modes.default.notifications]]
percentage = "30-"
power_supply = "None"
title = "Plug in!"
message = "The battery level of Your PC is low. Plug in."

# =======================================================

[[modes.RetainMoreCharged.notifications]]
percentage = "96+"
power_supply = "Adequate"
title = "Remove the plug!"
message = "Your PC is now fully charged. Remove the plug."


[[modes.RetainMoreCharged.notifications]]
percentage = "81-"
power_supply = "None"
title = "Plug in!"
message = "The battery level of Your PC is lower than 80%. Plug in."

# =======================================================

[[modes.RetainCharged.notifications]]
percentage = "96+"
power_supply = "Adequate"
title = "Remove the plug!"
message = "Your PC is now fully charged. Remove the plug."


[[modes.RetainCharged.notifications]]
percentage = "70-"
power_supply = "None"
title = "Plug in!"
message = "The battery level of Your PC is lower than 70%. Plug in."
```

## ToDos
- [x] ~Add mode change button~
- [x] ~Show mode selector button when this program starts~
- [ ] Implement notification with win32_notif
- [ ] Implement icon in taskbar with tray_icon
