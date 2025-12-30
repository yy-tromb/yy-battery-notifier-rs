Languages: [English](https://github.com/yy-tromb/yy-battery-notifier-rs) | 日本語(Japanese)

# yy-battery-notifier-rs
バッテリー残量に応じて通知を表示

> [!IMPORTANT]
> このアプリは現在***windows 10,11 のみ***に対応。これは WinRT API( Windows.Devices.Power.Battery と Windows.System.Power.PowerManager )を使っている関係と、[Rustのプラットフォームサポート](https://doc.rust-lang.org/beta/rustc/platform-support.html)によるもの。

> [!IMPORTANT]
> WinRT の notification API には Application User Model ID が必要。WIX インストーラでインストールする場合、Application User Model ID が Windows レジストリに設定されるため、すぐに通知可能。

## インストール

[ここ](https://github.com/yy-tromb/yy-battery-notifier-rs/releases)からダウンロード可能。MSI インストーラによるインストールを推奨。  
または、[Cargo](https://doc.rust-lang.org/cargo/) を使って以下のようにビルド...  
通常のリリースビルド: `cargo build --release` か `cargo b -r`  
gui featureを付けてビルド: `cargo build --release --features gui` か `cargo b -r --features gui`

## settings.toml について

TOML のフォーマットについてはググってください...

> [!NOTE]
> `TauriWinrtToast` メソッドは input 要素を実装していないため、`notification_method` が `TauriWinrtToast` のとき、それぞれの notification setting に対しての"input_type" フィールドは無視されます。

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

mode_names = [""]
# Optional field
# This field defines order of mode.
# Normally, the order of written in file is unsaved because of using hash.
# The mode not written in this field will be added to position later than modes written in this field.
# The mode not defined notifications will not be added to modes list.

initial_mode = ""
# Optional field
# This field defines mode (name of mode) to use initially.
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

### modes を使った設定
> [!IMPORTANT]
> `modes` 機能はバージョン 0.4.0 以降で利用可能。

```settings_mode_partial.toml
initial_mode = "default"
# Optional field
# This field defines mode (name of mode) to use initially.
# The default is modeless.

mode_names = ["default","RetainCharged","RetainMoreChanged"]
# Optional field
# This field defines order of mode.
# Normally, the order of written in file is unsaved because of using hash.
# The mode not written in this field will be added to position later than modes written in this field.
# The mode not defined notifications will not be added to modes list.

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
<summary>全てを展開</summary>

```settings_mode.toml
check_interval = 5
# interval seconds(integer) for check battery level.

notification_method = "TauriWinrtToast"
# Optional field
# This field defines notification method (notifier crate).
# Options now: "TauriWinrtToast"(default), "WinrtToastReborn"

mode_names = ["default","RetainCharged","RetainMoreChanged"]
# Optional field
# This field defines order of mode.
# Normally, the order of written in file is unsaved because of using hash.
# The mode not written in this field will be added to position later than modes written in this field.
# The mode not defined notifications will not be added to modes list.

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

## 通知の使い方
後で書く!

## CLIの使い方
### 指定された settings.toml の設定で通知を開始
`yy-battery-notifier-rs.exe -s "settings.toml へのパス"`

### デフォルトの設定 (default_settings.toml) で通知を開始
`yy-battery-notifier-rs.exe -d`

### サブコマンド
  aumid - register,delete : 通知に使われる Application User Model Id を Windows レジストリに登録、または削除。  
  startup - register,delete : このアプリをスタートアップに登録、または削除。 複数回登録した場合でも設定は上書きされる。

### オプション

  -s, --settings "settings.toml へのパス"  \[デフォルト: .\settings.toml]  
  -d, --default_settings : [default_settings.toml](https://github.com/yy-tromb/yy-battery-notifier-rs/blob/main/default_settings.toml) を使う  
      --msgbox : エラー発生をmsgboxで知らせる  
  -h, --help : ヘルプを表示  
  -V, --version : バージョンを表示  

## やること
- [x] ~モード変更ボタン~
- [x] ~起動時のモード選択~
- [ ] win32_notif クレートを使用した通知
- [ ] tray_icon クレートを使用してタスクバーにアイコンを表示
