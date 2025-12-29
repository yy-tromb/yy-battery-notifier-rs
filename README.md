> [!IMPORTANT]
> This application is now for ***windows 10,11 only***. This is because of using WinRT API( Windows.Devices.Power.Battery, Windows.System.Power.PowerManager ) and [the Rust's platform support](https://doc.rust-lang.org/beta/rustc/platform-support.html).

> [!IMPORTANT]
> The notification API of WinRT require Application User Model ID. If you install with WIX installer, Application User Model ID is set in Windows Registry and This app can toast.

# yy-battery-notifier-rs

check battery level and notify you

## Installation

You can install [from here](https://github.com/yy-tromb/yy-battery-notifier-rs/releases).
I recommend MSI installer.  
Or, you can build with [Cargo](https://doc.rust-lang.org/cargo/) as below...  
normal release build: `cargo build --release` or `cargo b -r`  
with gui feature: `cargo build --release --features gui` or `cargo b -r --features gui`

## About settings.toml

About TOML format, try google this...

> [!NOTE]
> "TauriWinrtToast" method does not implement input element, so "input_type" field of each notification setting is not effective.

### default_settings.toml

```default_settings.toml
check_interval = 60
# interval seconds(integer) for check battery level.

# ==================================================
# Optional fields below:

notification_method = "TauriWinrtToast"
# Optional field
# This field defines notification method (notifier crate).
# Options now: "TauriWinrtToast"(default), "WinrtToastReborn"

initial_mode = ""
# Optional field
# This field defines mode (name of mode) to use initially.
# The default is modeless ("").

abort_on_error_except_change_mode = false
# Optional field
# This field defines whether to abort on error except change mode.
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
# "notifications" is optional, but each field of a notification setting is required.
# This fields define notifications that always notify.

percentage = "90+"
# express condition when notifications should be started.
# This consists of percentage(integer) and suffix "+" or "-".
# This example express that notifications will start whether battery level excess 90%
#
power_supply = "Adequate"
# express power supply status. This consists of "Adequate", "InAdequate", "None".

title = "Remove the plug!"
# notification title

message = "Your PC is now fully charged. Remove the plug"
# notification message

input_type = "ModeSelector"
# Optional field
# This field defines type of input element of notification.
# Options: "ModeSelector"(default), "SilentSpecifiedMinutes"
# "TauriWinrtToast" method does not implement input element, so this field is not effective.

[[notifications]]
percentage = "45-"
power_supply = "None"
title = "Plug in!"
message = "The battery level of Your PC is low. Plug in."
```

### settings with modes
> [!IMPORTANT]
> `modes` features is not available with the current binary of GitHub releases.

```settings_mode_partial.toml
initial_mode = "default"
# Optional field
# This field defines mode (name of mode) to use initially.
# The default is modeless.

[[modes.default.notifications]]
# Name of mode is free, but you must follow the syntax of TOML.
# This fields define notifications that notify when mode is "default".

percentage = "92+"
# express condition when notifications should be started.
# This consists of percentage(integer) and suffix "+" or "-".
# This example express that notifications will start whether battery level excess 90%

power_supply = "Adequate"
# express power supply status. This consists of "Adequate", "InAdequate", "None".

title = "Remove the plug!"
# notification title

message = "Your PC is now fully charged. Remove the plug."
# notification message

input_type = "ModeSelector"
# Optional field
# This field defines type of input element of notification.
# Options: "ModeSelector"(default), "SilentSpecifiedMinutes"
# "TauriWinrtToast" method does not implement input element, so this field is not effective.
```

<details>
<summary>see full version</summary>

```settings_mode.toml
check_interval = 5
# interval seconds(integer) for check battery level.

notification_method = "TauriWinrtToast"
# Optional field
# This field defines notification method (notifier crate).
# Options now: "TauriWinrtToast"(default), "WinrtToastReborn"

initial_mode = "default"
# Optional field
# This field defines mode (name of mode) to use initially.
# The default is modeless.

# =======================================================

[[modes.default.notifications]]
# Name of mode is free, but you must follow the syntax of TOML.
# This fields define notifications that notify when mode is "default".

percentage = "92+"
# express condition when notifications should be started.
# This consists of percentage(integer) and suffix "+" or "-".
# This example express that notifications will start whether battery level excess 90%

power_supply = "Adequate"
# express power supply status. This consists of "Adequate", "InAdequate", "None".

title = "Remove the plug!"
# notification title

message = "Your PC is now fully charged. Remove the plug."
# notification message

input_type = "ModeSelector"
# Optional field
# This field defines type of input element of notification.
# Options: "ModeSelector"(default), "SilentSpecifiedMinutes"
# "TauriWinrtToast" method does not implement input element, so this field is not effective.

[[modes.default.notifications]]
percentage = "30-"
power_supply = "None"
title = "Plug in!"
message = "The battery level of Your PC is low. Plug in."

# =======================================================

[[modes.RetainMoreChanged.notifications]]
percentage = "96+"
power_supply = "Adequate"
title = "Remove the plug!"
message = "Your PC is now fully charged. Remove the plug."


[[modes.RetainMoreChanged.notifications]]
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

</details>

## Usage on notification
ToDo!

## Usage on CLI
### Start notify with specified settings.toml
`yy-battery-notifier-rs.exe -s "path to settings.toml"`

### Start notify with default settings
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

## ToDos
- [x] ~Add mode change button~
- [ ] Show mode selector button when this program starts
- [ ] Implement notification with win32_notif
- [ ] Implement icon in taskbar with tray_icon
