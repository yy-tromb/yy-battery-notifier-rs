// This is result of `cargo expand`
// You can check the result of macro expansion by seeing this.

#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use anyhow::Context as _;
use clap::Parser;
use colored::Colorize;
use hooq::hooq;
mod aumid {
    use crate::registry::{
        RegistryValue, check_deleted, check_registered, delete_tree, register,
    };
    use anyhow::Context as _;
    use colored::Colorize;
    use hooq::hooq;
    pub const AUMID: &str = "yy-tromb.yy-battery-notifier-rs";
    #[inline]
    pub fn register_and_check_aumid(root: &windows_registry::Key) -> anyhow::Result<()> {
        let keys_and_values = <[_]>::into_vec(
            ::alloc::boxed::box_new([
                (
                    "DisplayName",
                    RegistryValue::String("yy-battery-notifier-rs".to_string()),
                ),
                (
                    "IconUri",
                    RegistryValue::String(
                        std::env::current_exe()
                            .with_context(|| {
                                "Failed to get current execution file path."
                            })
                            .with_context(|| {
                                let path = "src\\aumid.rs";
                                let line = 15usize;
                                let col = 79usize;
                                let expr = "  14>    std::env::current_exe()\n  15|                .with_context(|| \"Fail..ath.\")?\n    |";
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                    )
                                })
                            })?
                            .to_str()
                            .unwrap_or(
                                r"C:\Program Files\yy-tromb\yy-battery-notifier-rs\yy-battery-notifier-rs.exe",
                            )
                            .to_string(),
                    ),
                ),
            ]),
        );
        register(
                root,
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("SOFTWARE\\Classes\\AppUserModelId\\{0}", AUMID),
                        )
                    })
                    .as_str(),
                &keys_and_values,
            )
            .with_context(|| {
                let path = "src\\aumid.rs";
                let line = 26usize;
                let col = 6usize;
                let expr = "  22>    register(\n  23|        root,\n  24|        format!(r\"SOFTWARE\\Classes\\AppUserModelId\\{}\", AUMID).as_str(),\n  25|        &keys_and_values,\n  26|    )?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        check_registered(
                root,
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("SOFTWARE\\Classes\\AppUserModelId\\{0}", AUMID),
                        )
                    })
                    .as_str(),
                &keys_and_values,
            )
            .with_context(|| {
                let path = "src\\aumid.rs";
                let line = 33usize;
                let col = 6usize;
                let expr = "  29>    check_registered(\n  30|        root,\n  31|        format!(r\"SOFTWARE\\Classes\\AppUserModelId\\{}\", AUMID).as_str(),\n  32|        &keys_and_values,\n  33|    )?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        {
            ::std::io::_print(
                format_args!("{0}\n", "register sucuessed!".green().on_black()),
            );
        };
        anyhow::Ok(())
    }
    #[inline]
    pub fn delete_and_check_aumid(root: &windows_registry::Key) -> anyhow::Result<()> {
        let keys = <[_]>::into_vec(::alloc::boxed::box_new(["DisplayName", "IconUri"]));
        delete_tree(root, r"SOFTWARE\Classes\AppUserModelId", AUMID)
            .with_context(|| {
                let path = "src\\aumid.rs";
                let line = 43usize;
                let col = 65usize;
                let expr = "  43>    delete_tree(root, \"SOFT..elId\", AUMID)?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        check_deleted(
                root,
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("SOFTWARE\\Classes\\AppUserModelId\\{0}", AUMID),
                        )
                    })
                    .as_str(),
                &keys,
            )
            .with_context(|| {
                let path = "src\\aumid.rs";
                let line = 48usize;
                let col = 6usize;
                let expr = "  44>    check_deleted(\n  45|        root,\n  46|        format!(r\"SOFTWARE\\Classes\\AppUserModelId\\{}\", AUMID).as_str(),\n  47|        &keys,\n  48|    )?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        {
            ::std::io::_print(
                format_args!("{0}\n", "delete sucuessed!".green().on_black()),
            );
        };
        anyhow::Ok(())
    }
}
mod battery {
    use anyhow::Context as _;
    use colored::Colorize;
    use hooq::hooq;
    pub struct BatteryReport {
        pub percentage: u32,
        pub remaining_seconds: Option<u64>,
        pub power_supply: PowerSupply,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BatteryReport {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "BatteryReport",
                "percentage",
                &self.percentage,
                "remaining_seconds",
                &self.remaining_seconds,
                "power_supply",
                &&self.power_supply,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for BatteryReport {
        #[inline]
        fn clone(&self) -> BatteryReport {
            BatteryReport {
                percentage: ::core::clone::Clone::clone(&self.percentage),
                remaining_seconds: ::core::clone::Clone::clone(&self.remaining_seconds),
                power_supply: ::core::clone::Clone::clone(&self.power_supply),
            }
        }
    }
    pub enum PowerSupply {
        Adequate,
        InAdequate,
        None,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PowerSupply {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    PowerSupply::Adequate => "Adequate",
                    PowerSupply::InAdequate => "InAdequate",
                    PowerSupply::None => "None",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PowerSupply {
        #[inline]
        fn clone(&self) -> PowerSupply {
            match self {
                PowerSupply::Adequate => PowerSupply::Adequate,
                PowerSupply::InAdequate => PowerSupply::InAdequate,
                PowerSupply::None => PowerSupply::None,
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PowerSupply {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PowerSupply {
        #[inline]
        fn eq(&self, other: &PowerSupply) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for PowerSupply {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl From<windows::System::Power::PowerSupplyStatus> for PowerSupply {
        fn from(value: windows::System::Power::PowerSupplyStatus) -> Self {
            use windows::System::Power::PowerSupplyStatus;
            match value {
                PowerSupplyStatus::Adequate => PowerSupply::Adequate,
                PowerSupplyStatus::Inadequate => PowerSupply::InAdequate,
                PowerSupplyStatus::NotPresent => PowerSupply::None,
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
    }
    pub fn battery_check() -> anyhow::Result<BatteryReport> {
        let mut battery_report = battery_check_winrt()
            .with_context(|| {
                let path = "src\\battery.rs";
                let line = 33usize;
                let col = 51usize;
                let expr = "  33>    battery_check_winrt()?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        match &battery_report {
            tmp => {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3} = {4:#?}\n",
                            "src\\battery.rs",
                            34u32,
                            5u32,
                            "& battery_report",
                            &&tmp as &dyn ::std::fmt::Debug,
                        ),
                    );
                };
                tmp
            }
        };
        if battery_report.remaining_seconds.is_none() {
            let batttery_report_win32 = battery_check_win32()
                .with_context(|| {
                    let path = "src\\battery.rs";
                    let line = 37usize;
                    let col = 58usize;
                    let expr = "  37>    battery_check_win32()?\n    |";
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                        )
                    })
                })?;
            match &batttery_report_win32 {
                tmp => {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "[{0}:{1}:{2}] {3} = {4:#?}\n",
                                "src\\battery.rs",
                                38u32,
                                9u32,
                                "& batttery_report_win32",
                                &&tmp as &dyn ::std::fmt::Debug,
                            ),
                        );
                    };
                    tmp
                }
            };
            if batttery_report_win32.remaining_seconds.is_some() {
                battery_report.remaining_seconds = batttery_report_win32
                    .remaining_seconds;
            }
            anyhow::Ok(battery_report)
        } else {
            anyhow::Ok(battery_report)
        }
            .with_context(|| {
                let path = "src\\battery.rs";
                let line = 36usize;
                let col = 5usize;
                let expr = "  36>    if battery_report.remaining_seconds.is_none() {\n...\n  42|        anyhow::Ok(battery_report)\n  43|    } else {\n  44|        anyhow::Ok(battery_report)\n  45|    }\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })
    }
    fn battery_check_winrt() -> anyhow::Result<BatteryReport> {
        use windows::System::Power::PowerManager;
        let percentage = PowerManager::RemainingChargePercent()
            .with_context(|| {
                let path = "src\\battery.rs";
                let line = 51usize;
                let col = 60usize;
                let expr = "  51>    PowerManager::RemainingChargePercent()?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })? as u32;
        let remaining_seconds = {
            let remaining_100_nano_seconds = PowerManager::RemainingDischargeTime()
                .with_context(|| {
                    let path = "src\\battery.rs";
                    let line = 53usize;
                    let col = 80usize;
                    let expr = "  53>    PowerManager::RemainingDischargeTime()?\n    |";
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                        )
                    })
                })?
                .Duration;
            if remaining_100_nano_seconds == i64::MAX {
                None
            } else {
                Some((remaining_100_nano_seconds / 10_000_000) as u64)
            }
        };
        let power_supply: PowerSupply = PowerManager::PowerSupplyStatus()
            .with_context(|| {
                let path = "src\\battery.rs";
                let line = 60usize;
                let col = 70usize;
                let expr = "  60>    PowerManager::PowerSupplyStatus()?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?
            .into();
        Ok(BatteryReport {
            percentage,
            remaining_seconds,
            power_supply,
        })
    }
    fn battery_check_win32() -> anyhow::Result<BatteryReport> {
        use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};
        let mut system_power_status = SYSTEM_POWER_STATUS::default();
        unsafe {
            GetSystemPowerStatus(&mut system_power_status)
                .with_context(|| {
                    let path = "src\\battery.rs";
                    let line = 74usize;
                    let col = 55usize;
                    let expr = "  74>    GetSystemPowerStatus(&mut system_power_status)?\n    |";
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                        )
                    })
                })?;
        }
        let percentage = match system_power_status.BatteryLifePercent {
            0..=100 => system_power_status.BatteryLifePercent as u32,
            255 => {
                return Err(
                        anyhow::Error::msg(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "{0}",
                                        "Battery life percentage is unknown.".red(),
                                    ),
                                )
                            }),
                        ),
                    )
                    .with_context(|| {
                        let path = "src\\battery.rs";
                        let line = 79usize;
                        let col = 13usize;
                        let expr = "  79>    return Err(anyhow::Error::msg(format!(\n  80|                \"{}\",\n  81|                \"Battery life percentage is unknown.\".red()\n  82|            )))\n    |";
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                            )
                        })
                    });
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        };
        let remaining_seconds = if system_power_status.BatteryLifeTime == u32::MAX {
            None
        } else {
            Some(system_power_status.BatteryLifeTime as u64)
        };
        let power_supply = match system_power_status.ACLineStatus {
            0 => PowerSupply::None,
            1 => PowerSupply::Adequate,
            255 => PowerSupply::InAdequate,
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        };
        Ok(BatteryReport {
            percentage,
            remaining_seconds,
            power_supply,
        })
    }
}
mod common {
    pub fn error_to_string(error: &anyhow::Error) -> String {
        let msg = ::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0:?}", error))
        });
        msg.replace("\x1b[31m", "").replace("\x1b[1m", "").replace("\x1b[0m", "")
    }
    pub fn error_msgbox(error: &anyhow::Error) -> anyhow::Result<()> {
        use windows::Win32::UI::WindowsAndMessaging::{
            MB_ICONERROR, MESSAGEBOX_RESULT, MessageBoxW,
        };
        use windows::core::HSTRING;
        const CARGO_PKG_NAME: &str = "yy-battery-notifier-rs";
        let title = HSTRING::from(CARGO_PKG_NAME);
        let content = HSTRING::from(error_to_string(error));
        match unsafe { MessageBoxW(None, &content, &title, MB_ICONERROR) } {
            MESSAGEBOX_RESULT(0) => {
                anyhow::Result::Err(
                        anyhow::Error::from(windows::core::Error::from_thread()),
                    )
                    .inspect_err(|e| {
                        let path = "src\\common.rs";
                        let line = 19usize;
                        let col = 13usize;
                        let expr = "  19>    anyhow::Result::Err(anyhow::Error::from(windows::core::Error::from_thread()))\n    |";
                        {
                            ::std::io::_eprint(
                                format_args!(
                                    "[{0}:{1}:{2}] {3:?}\n{4}\n",
                                    path,
                                    line,
                                    col,
                                    e,
                                    expr,
                                ),
                            );
                        };
                    })
            }
            _ => anyhow::Ok(()),
        }
            .inspect_err(|e| {
                let path = "src\\common.rs";
                let line = 17usize;
                let col = 5usize;
                let expr = "  17>    match unsafe { MessageBoxW(None, &content, &title, MB_ICONERROR) } {\n...\n  19|            anyhow::Result::Err(anyhow::Error::from(windows::core::Error::from_thread()))\n  20|        }\n  21|        _ => anyhow::Ok(()),\n  22|    }\n    |";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n{4}\n",
                            path,
                            line,
                            col,
                            e,
                            expr,
                        ),
                    );
                };
            })
    }
}
mod notification {
    mod tauri_winrt_toast {
        use anyhow::Context as _;
        use colored::Colorize;
        use hooq::hooq;
        use tauri_winrt_notification::{Duration, Progress, Toast};
        use std::sync::{Arc, Mutex};
        use super::NotificationAction;
        pub(super) fn battery_notify_tauri_winrt_toast(
            battery_report: &crate::battery::BatteryReport,
            title: &str,
            message: &str,
            notification_action: Arc<Mutex<Option<NotificationAction>>>,
        ) -> anyhow::Result<()> {
            let progress = match battery_report.remaining_seconds {
                Some(remaining_seconds) => {
                    Progress {
                        tag: "battery_progress".to_string(),
                        title: "Now Battery Level:".to_string(),
                        status: ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "{0}:{1}:{2} remaining",
                                    remaining_seconds / 3600,
                                    (remaining_seconds % 3600) / 60,
                                    remaining_seconds % 60,
                                ),
                            )
                        }),
                        value: battery_report.percentage as f32 / 100.0,
                        value_string: ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0}%", battery_report.percentage),
                            )
                        }),
                    }
                }
                None => {
                    Progress {
                        tag: "battery_progress".to_string(),
                        title: "Now Battery Level:".to_string(),
                        status: "Unknown time remaining".to_string(),
                        value: battery_report.percentage as f32 / 100.0,
                        value_string: ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0}%", battery_report.percentage),
                            )
                        }),
                    }
                }
            };
            let toast = Toast::new(crate::aumid::AUMID)
                .title(title)
                .text1(message)
                .progress(&progress)
                .add_button("silent for 5 mins", "silent 5 mins")
                .add_button("silent for 10 mins", "silent 10 mins")
                .add_button("change mode", "require change mode")
                .on_activated(move |action| {
                    handle_battery_notify_activated_action(action, &notification_action);
                    Ok(())
                })
                .duration(Duration::Short);
            toast
                .show()
                .map_err(|error| match error {
                    tauri_winrt_notification::Error::Os(e) => anyhow::Error::from(e),
                    tauri_winrt_notification::Error::Io(e) => anyhow::Error::from(e),
                })
                .with_context(|| {
                    let path = "src\\notification\\tauri_winrt_toast.rs";
                    let line = 53usize;
                    let col = 7usize;
                    let expr = "  50>    toast.show().map_err(|error| match error {\n  51|        tauri_winrt_notification::Error::Os(e) => anyhow::Error::from(e),\n  52|        tauri_winrt_notification::Error::Io(e) => anyhow::Error::from(e),\n  53|    })?\n    |";
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                        )
                    })
                })?;
            Ok(())
        }
        fn handle_battery_notify_activated_action(
            action: Option<String>,
            notification_action: &Arc<Mutex<Option<NotificationAction>>>,
        ) {
            match action.as_deref() {
                Some("silent 5 mins") => {
                    {
                        ::std::io::_print(
                            format_args!("Toast activated with action: silent 5 mins\n"),
                        );
                    };
                    if let Ok(mut action_guard) = notification_action.lock() {
                        *action_guard = Some(NotificationAction::Silent5Mins);
                    }
                }
                Some("silent 10 mins") => {
                    {
                        ::std::io::_print(
                            format_args!("Toast activated with action: silent 10 mins\n"),
                        );
                    };
                    if let Ok(mut action_guard) = notification_action.lock() {
                        *action_guard = Some(NotificationAction::Silent10Mins);
                    }
                }
                Some("require change mode") => {
                    {
                        ::std::io::_print(
                            format_args!(
                                "Toast activated with action: require change mode\n",
                            ),
                        );
                    };
                    if let Err(e) = crate::notification::mode_change_notify(
                        &crate::notification::NotificationMethod::TauriWinrtToast,
                        Arc::clone(notification_action),
                    ) {
                        let mut action_guard = match notification_action.lock() {
                            Ok(guard) => guard,
                            Err(e) => e.into_inner(),
                        };
                        *action_guard = Some(NotificationAction::Error(e));
                        drop(action_guard);
                    }
                    if let Ok(mut action_guard) = notification_action.lock() {
                        *action_guard = Some(NotificationAction::RequireChangeMode);
                    }
                }
                Some(unknown_action) => {
                    {
                        ::std::io::_print(
                            format_args!(
                                "Toast activated with unknown action: {0}\n",
                                unknown_action,
                            ),
                        );
                    };
                }
                None => {
                    {
                        ::std::io::_print(
                            format_args!("Toast activated without action.\n"),
                        );
                    };
                }
            }
        }
        pub(super) fn mode_change_notify_tauri_winrt_toast(
            notification_action: Arc<Mutex<Option<NotificationAction>>>,
        ) -> anyhow::Result<()> {
            let mut toast = Toast::new(crate::aumid::AUMID)
                .title("Notify Mode Change")
                .duration(Duration::Long)
                .add_button("&lt;no mode&gt;", "mode_no_mode");
            for mode_name in crate::runner::MODE_NAMES
                .get()
                .ok_or_else(|| {
                    anyhow::Error::msg(
                        "MODE_NAMES is not initilized. This can not happen.".red(),
                    )
                })
                .with_context(|| {
                    let path = "src\\notification\\tauri_winrt_toast.rs";
                    let line = 110usize;
                    let col = 7usize;
                    let expr = " 108>    crate::runner::MODE_NAMES.get().ok_or_else(|| {\n 109|        anyhow::Error::msg(\"MODE..pen.\".red())\n 110|    })?\n    |";
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                        )
                    })
                })?
            {
                toast = toast
                    .add_button(
                        mode_name,
                        ::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("mode:{0}", mode_name))
                            })
                            .as_str(),
                    );
            }
            toast = toast
                .on_activated(move |action| {
                    handle_mode_change_notify_activated_action(
                        action,
                        &notification_action,
                    );
                    Ok(())
                });
            toast
                .show()
                .map_err(|error| match error {
                    tauri_winrt_notification::Error::Os(e) => anyhow::Error::from(e),
                    tauri_winrt_notification::Error::Io(e) => anyhow::Error::from(e),
                })
                .with_context(|| {
                    let path = "src\\notification\\tauri_winrt_toast.rs";
                    let line = 122usize;
                    let col = 7usize;
                    let expr = " 119>    toast.show().map_err(|error| match error {\n 120|        tauri_winrt_notification::Error::Os(e) => anyhow::Error::from(e),\n 121|        tauri_winrt_notification::Error::Io(e) => anyhow::Error::from(e),\n 122|    })?\n    |";
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                        )
                    })
                })?;
            Ok(())
        }
        fn handle_mode_change_notify_activated_action(
            action: Option<String>,
            notification_action: &Arc<Mutex<Option<NotificationAction>>>,
        ) {
            match action.as_deref() {
                Some("mode_no_mode") => {
                    {
                        ::std::io::_print(format_args!("change to no mode\n"));
                    };
                    let mut guard = match notification_action.lock() {
                        Ok(guard) => guard,
                        Err(e) => e.into_inner(),
                    };
                    *guard = Some(NotificationAction::ChangeMode(String::default()));
                }
                Some(action) => {
                    {
                        ::std::io::_print(
                            format_args!("change mode to id=\"{0}\"\n", action),
                        );
                    };
                    let mut guard = match notification_action.lock() {
                        Ok(guard) => guard,
                        Err(e) => e.into_inner(),
                    };
                    *guard = Some(
                        NotificationAction::ChangeMode(
                            action.get(5..).unwrap_or_default().to_string(),
                        ),
                    );
                }
                None => {
                    {
                        ::std::io::_print(
                            format_args!("Toast activated without action.\n"),
                        );
                    };
                }
            }
        }
    }
    mod win32_notif {
        #[allow(unused)]
        use anyhow::Context as _;
        #[allow(unused)]
        use hooq::hooq;
        #[allow(unused)]
        use win32_notif::{Notification, NotificationDataSet, ToastsNotifier};
        #[allow(unused)]
        use std::sync::{Arc, Mutex};
        #[allow(unused)]
        use super::NotificationAction;
    }
    mod winrt_toast_reborn {
        use anyhow::Context as _;
        use colored::Colorize;
        use hooq::hooq;
        use winrt_toast_reborn::content::input::InputType;
        use winrt_toast_reborn::{
            Action, Input, Selection, Toast, ToastDuration, ToastManager,
        };
        use std::sync::{Arc, Mutex};
        use super::{NotificationAction, NotificationInputType};
        pub(super) fn battery_notify_winrt_toast_reborn(
            battery_report: &crate::battery::BatteryReport,
            title: &str,
            message: &str,
            notification_action: Arc<Mutex<Option<NotificationAction>>>,
            input_type: &NotificationInputType,
            mode_names: &[String],
        ) -> anyhow::Result<()> {
            let toast_manager = ToastManager::new(crate::aumid::AUMID);
            let battery_remaining_status = match battery_report.remaining_seconds {
                Some(remaining_seconds) => {
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{0}:{1}:{2} remaining",
                                remaining_seconds / 3600,
                                (remaining_seconds % 3600) / 60,
                                remaining_seconds % 60,
                            ),
                        )
                    })
                }
                None => "Unknown time remaining.".to_string(),
            };
            let mut toast = Toast::new();
            toast
                .text1(title)
                .text2(message)
                .text3(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "Battery level: {0}%, {1}",
                                battery_report.percentage,
                                battery_remaining_status,
                            ),
                        )
                    }),
                );
            toast
                .duration(ToastDuration::Short)
                .action(Action::new("silent for 5 mins", "silent 5 mins", ""))
                .action(Action::new("silent for 10 mins", "silent 10 mins", ""));
            let mode_guard = match crate::runner::MODE.read() {
                Ok(mode) => mode,
                Err(e) => e.into_inner(),
            };
            match input_type {
                NotificationInputType::ModeSelector if !mode_names.is_empty() => {
                    toast
                        .input(
                            Input::new("mode_selection", InputType::Selection)
                                .with_title("select mode")
                                .with_default_input(
                                    if mode_guard.is_empty() {
                                        "mode_no_mode".into()
                                    } else {
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(format_args!("mode:{0}", mode_guard))
                                        })
                                    },
                                ),
                        );
                    toast.selection(Selection::new("mode_no_mode", "<no mode>"));
                    mode_names
                        .iter()
                        .map(|mode_name| Selection::new(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("mode:{0}", mode_name))
                            }),
                            mode_name,
                        ))
                        .for_each(|selection| {
                            toast.selection(selection);
                        });
                    toast
                        .action(
                            Action::new("change mode", "change mode", "")
                                .with_input_id("mode_selection"),
                        );
                }
                NotificationInputType::ModeSelector
                | NotificationInputType::SilentSpecifiedMinutes => {
                    toast
                        .input(
                            Input::new("silent_time", InputType::Text)
                                .with_title("Input silent minites:")
                                .with_default_input("5"),
                        )
                        .action(
                            Action::new("mins: Keep silent", "silent specified mins", "")
                                .with_input_id("silent_time"),
                        )
                        .action(Action::new("change mode", "require change mode", ""));
                }
            }
            drop(mode_guard);
            toast_manager
                .on_activated(
                    None,
                    move |action| {
                        handle_battery_notify_activated_action(
                            action,
                            &notification_action,
                        );
                    },
                )
                .show(&toast)
                .with_context(|| {
                    let path = "src\\notification\\winrt_toast_reborn.rs";
                    let line = 97usize;
                    let col = 22usize;
                    let expr = "  93>    toast_manager\n  94|        .on_activated(None, move |action| {\n  95|            handle_battery_notify_activated_action(action, &notification_action);\n  96|        })\n  97|        .show(&toast)?\n    |";
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                        )
                    })
                })?;
            Ok(())
        }
        fn handle_battery_notify_activated_action(
            action: Option<winrt_toast_reborn::ActivatedAction>,
            notification_action: &Arc<Mutex<Option<NotificationAction>>>,
        ) {
            match action {
                Some(action) => {
                    let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("Toast activated with action: {0}", action.arg),
                        )
                    });
                    {
                        ::std::io::_print(format_args!("{0}\n", message));
                    };
                    match action.arg.as_str() {
                        "silent 5 mins" => {
                            if let Ok(mut guard) = notification_action.lock() {
                                {
                                    ::std::io::_print(
                                        format_args!("Setting silent time to 5 mins\n"),
                                    );
                                };
                                *guard = Some(NotificationAction::Silent5Mins);
                            }
                        }
                        "silent 10 mins" => {
                            if let Ok(mut guard) = notification_action.lock() {
                                *guard = Some(NotificationAction::Silent10Mins);
                            }
                        }
                        "silent specified mins" => {
                            if let Some(silent_time) = action.values.get("silent_time") {
                                if let Ok(silent_time_int) = silent_time.parse::<u64>() {
                                    let mut guard = match notification_action.lock() {
                                        Ok(guard) => guard,
                                        Err(e) => e.into_inner(),
                                    };
                                    {
                                        ::std::io::_print(
                                            format_args!(
                                                "Setting silent time to: {0}\n",
                                                silent_time_int,
                                            ),
                                        );
                                    };
                                    *guard = Some(
                                        NotificationAction::SilentSpecifiedMins(silent_time_int),
                                    );
                                } else {
                                    {
                                        ::std::io::_print(
                                            format_args!(
                                                "Failed to parse silent time input: {0}\n",
                                                silent_time,
                                            ),
                                        );
                                    };
                                }
                            } else {
                                {
                                    ::std::io::_print(
                                        format_args!("No input value found for silent time.\n"),
                                    );
                                };
                            }
                        }
                        #[allow(clippy::collapsible_if)]
                        "require change mode" => {
                            {
                                ::std::io::_print(format_args!("Required change mode\n"));
                            };
                            if let Err(e) = crate::notification::mode_change_notify(
                                &crate::notification::NotificationMethod::WinrtToastReborn,
                                Arc::clone(notification_action),
                            ) {
                                if let Ok(mut action_guard) = notification_action.lock() {
                                    *action_guard = Some(NotificationAction::Error(e));
                                    drop(action_guard);
                                }
                            }
                            let mut guard = match notification_action.lock() {
                                Ok(guard) => guard,
                                Err(e) => e.into_inner(),
                            };
                            *guard = Some(NotificationAction::RequireChangeMode);
                        }
                        "change mode" => {
                            if let Some(mode_to_change) = action
                                .values
                                .get("mode_selection")
                            {
                                let mut guard = match notification_action.lock() {
                                    Ok(guard) => guard,
                                    Err(e) => e.into_inner(),
                                };
                                if mode_to_change == "mode_no_mode" {
                                    {
                                        ::std::io::_print(format_args!("change to no mode\n"));
                                    };
                                    *guard = Some(
                                        NotificationAction::ChangeMode(String::default()),
                                    );
                                } else {
                                    {
                                        ::std::io::_print(
                                            format_args!("change mode to id=\"{0}\"\n", mode_to_change),
                                        );
                                    };
                                    *guard = Some(
                                        NotificationAction::ChangeMode(
                                            mode_to_change.get(5..).unwrap_or_default().to_string(),
                                        ),
                                    );
                                }
                            } else {
                                {
                                    ::std::io::_print(
                                        format_args!("No input value found for change mode.\n"),
                                    );
                                };
                            }
                        }
                        _ => {
                            {
                                ::std::io::_print(format_args!("Unknown action.\n"));
                            };
                        }
                    }
                }
                None => {
                    {
                        ::std::io::_print(
                            format_args!("Toast activated without action.\n"),
                        );
                    };
                }
            }
        }
        pub(super) fn mode_change_notify_winrt_toast_reborn(
            notification_action: Arc<Mutex<Option<NotificationAction>>>,
        ) -> anyhow::Result<()> {
            let toast_manager = ToastManager::new(crate::aumid::AUMID);
            let mut toast = Toast::new();
            let mode_guard = match crate::runner::MODE.read() {
                Ok(mode) => mode,
                Err(e) => e.into_inner(),
            };
            toast
                .text1("Notify Mode Change")
                .duration(ToastDuration::Long)
                .input(
                    Input::new("mode_selection", InputType::Selection)
                        .with_title("select mode")
                        .with_default_input(
                            if mode_guard.is_empty() {
                                "mode_no_mode".into()
                            } else {
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(format_args!("mode:{0}", mode_guard))
                                })
                            },
                        ),
                )
                .selection(Selection::new("mode_no_mode", "<no mode>"));
            drop(mode_guard);
            crate::runner::MODE_NAMES
                .get()
                .ok_or_else(|| {
                    anyhow::Error::msg(
                        "MODE_NAMES is not initilized. This can not happen.".red(),
                    )
                })
                .with_context(|| {
                    let path = "src\\notification\\winrt_toast_reborn.rs";
                    let line = 213usize;
                    let col = 11usize;
                    let expr = " 209>    crate::runner::MODE_NAMES\n 210|        .get()\n 211|        .ok_or_else(|| {\n 212|            anyhow::Error::msg(\"MODE..pen.\".red())\n 213|        })?\n    |";
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                        )
                    })
                })?
                .iter()
                .map(|mode_name| Selection::new(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("mode:{0}", mode_name))
                    }),
                    mode_name,
                ))
                .for_each(|selection| {
                    toast.selection(selection);
                });
            toast
                .action(
                    Action::new("change mode", "change mode", "")
                        .with_input_id("mode_selection"),
                );
            toast_manager
                .on_activated(
                    None,
                    move |action| {
                        handle_mode_change_notify_winrt_toast_reborn(
                            action,
                            &notification_action,
                        );
                    },
                )
                .show(&toast)
                .with_context(|| {
                    let path = "src\\notification\\winrt_toast_reborn.rs";
                    let line = 224usize;
                    let col = 22usize;
                    let expr = " 220>    toast_manager\n 221|        .on_activated(None, move |action| {\n 222|            handle_mode_change_notify_winrt_toast_reborn(action, &notification_action);\n 223|        })\n 224|        .show(&toast)?\n    |";
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                        )
                    })
                })?;
            Ok(())
        }
        fn handle_mode_change_notify_winrt_toast_reborn(
            action: Option<winrt_toast_reborn::ActivatedAction>,
            notification_action: &Arc<Mutex<Option<NotificationAction>>>,
        ) {
            match action {
                Some(action) => {
                    let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("Toast activated with action: {0}", action.arg),
                        )
                    });
                    {
                        ::std::io::_print(format_args!("{0}\n", message));
                    };
                    match action.arg.as_str() {
                        #[allow(clippy::collapsible_if)]
                        "change mode" => {
                            if let Some(mode_to_change) = action
                                .values
                                .get("mode_selection")
                            {
                                let mut guard = match notification_action.lock() {
                                    Ok(guard) => guard,
                                    Err(e) => e.into_inner(),
                                };
                                if mode_to_change == "mode_no_mode" {
                                    {
                                        ::std::io::_print(format_args!("change to no mode\n"));
                                    };
                                    *guard = Some(
                                        NotificationAction::ChangeMode(String::default()),
                                    );
                                } else {
                                    {
                                        ::std::io::_print(
                                            format_args!("change mode to id=\"{0}\"\n", mode_to_change),
                                        );
                                    };
                                    *guard = Some(
                                        NotificationAction::ChangeMode(
                                            mode_to_change.get(5..).unwrap_or_default().to_string(),
                                        ),
                                    );
                                }
                            } else {
                                {
                                    ::std::io::_print(
                                        format_args!("No input value found for change mode.\n"),
                                    );
                                };
                            }
                        }
                        _ => {
                            {
                                ::std::io::_print(format_args!("Unknown action.\n"));
                            };
                        }
                    }
                }
                None => {
                    {
                        ::std::io::_print(
                            format_args!("Toast activated without action.\n"),
                        );
                    };
                }
            }
        }
    }
    use anyhow::Context as _;
    use hooq::hooq;
    use std::sync::{Arc, Mutex};
    pub enum NotificationMethod {
        #[default]
        TauriWinrtToast,
        WinrtToastReborn,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for NotificationMethod {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    NotificationMethod::TauriWinrtToast => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "NotificationMethod",
                            0u32,
                            "TauriWinrtToast",
                        )
                    }
                    NotificationMethod::WinrtToastReborn => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "NotificationMethod",
                            1u32,
                            "WinrtToastReborn",
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for NotificationMethod {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            _ => {
                                _serde::__private228::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 2",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "TauriWinrtToast" => {
                                _serde::__private228::Ok(__Field::__field0)
                            }
                            "WinrtToastReborn" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            _ => {
                                _serde::__private228::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"TauriWinrtToast" => {
                                _serde::__private228::Ok(__Field::__field0)
                            }
                            b"WinrtToastReborn" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            _ => {
                                let __value = &_serde::__private228::from_utf8_lossy(
                                    __value,
                                );
                                _serde::__private228::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<NotificationMethod>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = NotificationMethod;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "enum NotificationMethod",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private228::Ok(
                                    NotificationMethod::TauriWinrtToast,
                                )
                            }
                            (__Field::__field1, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private228::Ok(
                                    NotificationMethod::WinrtToastReborn,
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[
                    "TauriWinrtToast",
                    "WinrtToastReborn",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "NotificationMethod",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<NotificationMethod>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for NotificationMethod {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    NotificationMethod::TauriWinrtToast => "TauriWinrtToast",
                    NotificationMethod::WinrtToastReborn => "WinrtToastReborn",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for NotificationMethod {
        #[inline]
        fn clone(&self) -> NotificationMethod {
            match self {
                NotificationMethod::TauriWinrtToast => {
                    NotificationMethod::TauriWinrtToast
                }
                NotificationMethod::WinrtToastReborn => {
                    NotificationMethod::WinrtToastReborn
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for NotificationMethod {
        #[inline]
        fn default() -> NotificationMethod {
            Self::TauriWinrtToast
        }
    }
    pub enum NotificationInputType {
        #[default]
        ModeSelector,
        SilentSpecifiedMinutes,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for NotificationInputType {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    NotificationInputType::ModeSelector => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "NotificationInputType",
                            0u32,
                            "ModeSelector",
                        )
                    }
                    NotificationInputType::SilentSpecifiedMinutes => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "NotificationInputType",
                            1u32,
                            "SilentSpecifiedMinutes",
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for NotificationInputType {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            _ => {
                                _serde::__private228::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 2",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "ModeSelector" => _serde::__private228::Ok(__Field::__field0),
                            "SilentSpecifiedMinutes" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            _ => {
                                _serde::__private228::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"ModeSelector" => {
                                _serde::__private228::Ok(__Field::__field0)
                            }
                            b"SilentSpecifiedMinutes" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            _ => {
                                let __value = &_serde::__private228::from_utf8_lossy(
                                    __value,
                                );
                                _serde::__private228::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<NotificationInputType>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = NotificationInputType;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "enum NotificationInputType",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private228::Ok(
                                    NotificationInputType::ModeSelector,
                                )
                            }
                            (__Field::__field1, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private228::Ok(
                                    NotificationInputType::SilentSpecifiedMinutes,
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[
                    "ModeSelector",
                    "SilentSpecifiedMinutes",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "NotificationInputType",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<
                            NotificationInputType,
                        >,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for NotificationInputType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    NotificationInputType::ModeSelector => "ModeSelector",
                    NotificationInputType::SilentSpecifiedMinutes => {
                        "SilentSpecifiedMinutes"
                    }
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for NotificationInputType {
        #[inline]
        fn clone(&self) -> NotificationInputType {
            match self {
                NotificationInputType::ModeSelector => {
                    NotificationInputType::ModeSelector
                }
                NotificationInputType::SilentSpecifiedMinutes => {
                    NotificationInputType::SilentSpecifiedMinutes
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for NotificationInputType {
        #[inline]
        fn default() -> NotificationInputType {
            Self::ModeSelector
        }
    }
    pub enum NotificationAction {
        Silent5Mins,
        Silent10Mins,
        SilentSpecifiedMins(u64),
        RequireChangeMode,
        ChangeMode(String),
        Error(anyhow::Error),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for NotificationAction {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                NotificationAction::Silent5Mins => {
                    ::core::fmt::Formatter::write_str(f, "Silent5Mins")
                }
                NotificationAction::Silent10Mins => {
                    ::core::fmt::Formatter::write_str(f, "Silent10Mins")
                }
                NotificationAction::SilentSpecifiedMins(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "SilentSpecifiedMins",
                        &__self_0,
                    )
                }
                NotificationAction::RequireChangeMode => {
                    ::core::fmt::Formatter::write_str(f, "RequireChangeMode")
                }
                NotificationAction::ChangeMode(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ChangeMode",
                        &__self_0,
                    )
                }
                NotificationAction::Error(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Error",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[inline]
    pub fn judge_notification(
        notification_setting: &crate::settings::NotificationSetting,
        battery_report: &crate::battery::BatteryReport,
    ) -> bool {
        match notification_setting.percentage_symbol {
            crate::settings::PercentageSymbol::Excess => {
                (battery_report.percentage > notification_setting.percentage_int)
                    && (battery_report.power_supply == notification_setting.power_supply)
            }
            crate::settings::PercentageSymbol::Under => {
                (battery_report.percentage < notification_setting.percentage_int)
                    && (battery_report.power_supply == notification_setting.power_supply)
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn battery_notify(
        method: &NotificationMethod,
        battery_report: &crate::battery::BatteryReport,
        title: &str,
        message: &str,
        notification_action: Arc<Mutex<Option<NotificationAction>>>,
        input_type: &NotificationInputType,
        mode_names: &[String],
    ) -> anyhow::Result<()> {
        match method {
            NotificationMethod::TauriWinrtToast => {
                tauri_winrt_toast::battery_notify_tauri_winrt_toast(
                    battery_report,
                    title,
                    message,
                    notification_action,
                )
            }
            NotificationMethod::WinrtToastReborn => {
                winrt_toast_reborn::battery_notify_winrt_toast_reborn(
                    battery_report,
                    title,
                    message,
                    notification_action,
                    input_type,
                    mode_names,
                )
            }
        }
            .with_context(|| {
                let path = "src\\notification\\mod.rs";
                let line = 63usize;
                let col = 5usize;
                let expr = "  63>    match method {\n...\n  77|                mode_names,\n  78|            )\n  79|        }\n  80|    }\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })
    }
    #[inline]
    pub fn mode_change_notify(
        method: &NotificationMethod,
        notification_action: Arc<Mutex<Option<NotificationAction>>>,
    ) -> anyhow::Result<()> {
        match method {
            NotificationMethod::TauriWinrtToast => {
                tauri_winrt_toast::mode_change_notify_tauri_winrt_toast(
                    notification_action,
                )
            }
            NotificationMethod::WinrtToastReborn => {
                winrt_toast_reborn::mode_change_notify_winrt_toast_reborn(
                    notification_action,
                )
            }
        }
            .with_context(|| {
                let path = "src\\notification\\mod.rs";
                let line = 89usize;
                let col = 5usize;
                let expr = "  89>    match method {\n...\n  93|        NotificationMethod::WinrtToastReborn => {\n  94|            winrt_toast_reborn::mode_change_notify_winrt_toast_reborn(notification_action)\n  95|        }\n  96|    }\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })
    }
}
mod registry {
    use anyhow::Context as _;
    use colored::Colorize;
    use hooq::hooq;
    pub use windows_registry::{CURRENT_USER, LOCAL_MACHINE};
    pub enum RegistryValue<'a> {
        String(String),
        Bytes(&'a [u8]),
    }
    const WIN32_ERROR_E_FILENOTFOUND: windows::core::HRESULT = windows::core::HRESULT::from_win32(
        0x80070002,
    );
    #[allow(unused)]
    const WIN32_ERROR_E_ACCESSDENIED: windows::core::HRESULT = windows::core::HRESULT::from_win32(
        0x80070005,
    );
    #[inline]
    fn str_to_wide(s: &str) -> windows::core::PCWSTR {
        windows::core::PCWSTR::from_raw(
            s.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>().as_ptr(),
        )
    }
    pub fn register(
        root: &windows_registry::Key,
        path: &str,
        keys_and_values: &[(&str, RegistryValue)],
    ) -> anyhow::Result<()> {
        let tree = root
            .create(path)
            .with_context(|| {
                if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                    ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Failed to open registry \'HKEY_LOCAL_MACHINE\\{0}\'. You may need to run as administrator.",
                                    path,
                                ),
                            )
                        })
                        .red()
                } else {
                    ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Failed to open registry \'HKEY_CURRENT_USER\\{0}\'.",
                                    path,
                                ),
                            )
                        })
                        .red()
                }
            })
            .with_context(|| {
                let path = "src\\registry.rs";
                let line = 41usize;
                let col = 11usize;
                let expr = "  33>    root\n...\n  38|            } else {\n  39|                format!(\"Failed to open registry 'HKEY_CURRENT_USER\\\\{}'.\", path).red()\n  40|            }\n  41|        })?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        for (key, value) in keys_and_values {
            match value {
                RegistryValue::String(value_string) => {
                    tree.set_string(key, value_string)
                        .with_context(|| {
                            if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw()
                            {
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to set registry \'HKEY_LOCAL_MACHINE\\{0}\\{1}\'.",
                                            path,
                                            key,
                                        ),
                                    )
                                })
                            } else {
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to set registry \'HKEY_CURRENT_USER\\{0}\\{1}\'.",
                                            path,
                                            key,
                                        ),
                                    )
                                })
                            }
                        })
                        .with_context(|| {
                            let path = "src\\registry.rs";
                            let line = 57usize;
                            let col = 19usize;
                            let expr = "  45>    tree.set_string(key, value_string).with_context(|| {\n...\n  54|                            path, key\n  55|                        )\n  56|                    }\n  57|                })?\n    |";
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                )
                            })
                        })?;
                }
                RegistryValue::Bytes(value_bytes) => {
                    tree.set_bytes(key, windows_registry::Type::Bytes, value_bytes)
                        .with_context(|| {
                            if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw()
                            {
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to set registry \'HKEY_LOCAL_MACHINE\\{0}\\{1}\'.",
                                            path,
                                            key,
                                        ),
                                    )
                                })
                            } else {
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to set registry \'HKEY_CURRENT_USER\\{0}\\{1}\'.",
                                            path,
                                            key,
                                        ),
                                    )
                                })
                            }
                        })
                        .with_context(|| {
                            let path = "src\\registry.rs";
                            let line = 73usize;
                            let col = 23usize;
                            let expr = "  60>    tree.set_bytes(key, windows_registry::Type::Bytes, value_bytes)\n...\n  70|                                path, key\n  71|                            )\n  72|                        }\n  73|                    })?\n    |";
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                )
                            })
                        })?;
                }
            }
        }
        anyhow::Ok(())
    }
    pub fn check_registered(
        root: &windows_registry::Key,
        path: &str,
        keys_and_values: &[(&str, RegistryValue)],
    ) -> anyhow::Result<()> {
        let tree = root
            .create(path)
            .with_context(|| {
                if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                    ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Failed to open registry \'HKEY_LOCAL_MACHINE\\{0}\'. You may need to run as administrator.",
                                    path,
                                ),
                            )
                        })
                        .red()
                } else {
                    ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Failed to open registry \'HKEY_CURRENT_USER\\{0}\'.",
                                    path,
                                ),
                            )
                        })
                        .red()
                }
            })
            .with_context(|| {
                let path = "src\\registry.rs";
                let line = 94usize;
                let col = 11usize;
                let expr = "  86>    root\n...\n  91|            } else {\n  92|                format!(r\"Failed to open registry 'HKEY_CURRENT_USER\\{}'.\",path).red()\n  93|            }\n  94|        })?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        for (key, value) in keys_and_values {
            match value {
                RegistryValue::String(value_string) => {
                    let read_value = tree
                        .get_string(key)
                        .with_context(|| {
                            if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw()
                            {
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to read registry \'HKEY_LOCAL_MACHINE\\{0}\\{1}\'.",
                                            path,
                                            key,
                                        ),
                                    )
                                })
                            } else {
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to read registry \'HKEY_CURRENT_USER\\{0}\\{1}\'.",
                                            path,
                                            key,
                                        ),
                                    )
                                })
                            }
                        })
                        .with_context(|| {
                            let path = "src\\registry.rs";
                            let line = 110usize;
                            let col = 19usize;
                            let expr = "  98>    tree.get_string(key).with_context(|| {\n...\n 107|                            path, key\n 108|                        )\n 109|                    }\n 110|                })?\n    |";
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                )
                            })
                        })?;
                    if &read_value != value_string {
                        return anyhow::Result::Err(
                                anyhow::Error::msg(
                                    ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "Registry \'HKEY_CURRENT_USER\\{0}\\{1}\' has unexpected value. Expected: {2}, Found: {3}",
                                                    path,
                                                    key,
                                                    value_string,
                                                    read_value,
                                                ),
                                            )
                                        })
                                        .red(),
                                ),
                            )
                            .with_context(|| {
                                let path = "src\\registry.rs";
                                let line = 112usize;
                                let col = 21usize;
                                let expr = " 112>    return anyhow::Result::Err(anyhow::Error::msg(format!(\n...\n 115|                        key,\n 116|                        value_string,\n 117|                        read_value\n 118|                    ).red()))\n    |";
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                    )
                                })
                            });
                    }
                }
                RegistryValue::Bytes(value_bytes) => {
                    let mut buffer: Vec<u8> = ::alloc::vec::from_elem(0, 260);
                    let key_wide = str_to_wide(key);
                    let read_value_info = unsafe {
                        tree.raw_get_bytes(key_wide, &mut buffer)
                            .with_context(|| {
                                if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw()
                                {
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Failed to read registry \'HKEY_LOCAL_MACHINE\\{0}\\{1}\'.",
                                                path,
                                                key,
                                            ),
                                        )
                                    })
                                } else {
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Failed to read registry \'HKEY_CURRENT_USER\\{0}\\{1}\'.",
                                                path,
                                                key,
                                            ),
                                        )
                                    })
                                }
                            })
                            .with_context(|| {
                                let path = "src\\registry.rs";
                                let line = 137usize;
                                let col = 23usize;
                                let expr = " 125>    tree.raw_get_bytes(key_wide, &mut buffer).with_context(|| {\n...\n 134|                                path, key\n 135|                            )\n 136|                        }\n 137|                    })?\n    |";
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                    )
                                })
                            })?
                    };
                    if read_value_info.0 != windows_registry::Type::Bytes
                        || read_value_info.1 != *value_bytes
                    {
                        if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                            return anyhow::Result::Err(
                                    anyhow::Error::msg(
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!(
                                                        "\'HKEY_LOCAL_MACHINE\\{0}\\{1}\' has unexpected value.\\n\\\n                    Except: {2:?} \\n\\\n                    But Found {3:?}",
                                                        path,
                                                        key,
                                                        value_bytes,
                                                        &read_value_info.1,
                                                    ),
                                                )
                                            })
                                            .red(),
                                    ),
                                )
                                .with_context(|| {
                                    let path = "src\\registry.rs";
                                    let line = 143usize;
                                    let col = 25usize;
                                    let expr = " 143>    return anyhow::Result::Err(anyhow::Error::msg(\n...\n 150|                                path, key, value_bytes, &read_value_info.1\n 151|                            )\n 152|                            .red(),\n 153|                        ))\n    |";
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                        )
                                    })
                                });
                        } else {
                            return anyhow::Result::Err(
                                    anyhow::Error::msg(
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!(
                                                        "\'HKEY_CURRENT_USER\\{0}\\{1}\' has unexpected value.\\n\\\n                    Except: {2:?} \\n\\\n                    But Found {3:?}",
                                                        path,
                                                        key,
                                                        value_bytes,
                                                        &read_value_info.1,
                                                    ),
                                                )
                                            })
                                            .red(),
                                    ),
                                )
                                .with_context(|| {
                                    let path = "src\\registry.rs";
                                    let line = 153usize;
                                    let col = 25usize;
                                    let expr = " 153>    return anyhow::Result::Err(anyhow::Error::msg(\n...\n 160|                                path, key, value_bytes, &read_value_info.1\n 161|                            )\n 162|                            .red(),\n 163|                        ))\n    |";
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                        )
                                    })
                                });
                        }
                    }
                }
            }
        }
        anyhow::Ok(())
    }
    pub fn delete_tree(
        root: &windows_registry::Key,
        parent_path: &str,
        target_tree: &str,
    ) -> anyhow::Result<()> {
        let tree = root
            .create(parent_path)
            .with_context(|| {
                if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                    ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Failed to open registry \'HKEY_LOCAL_MACHINE\\{0}\'. You may need to run as administrator.",
                                    parent_path,
                                ),
                            )
                        })
                        .red()
                } else {
                    ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Failed to open registry \'HKEY_CURRENT_USER\\{0}\'.",
                                    parent_path,
                                ),
                            )
                        })
                        .red()
                }
            })
            .with_context(|| {
                let path = "src\\registry.rs";
                let line = 184usize;
                let col = 11usize;
                let expr = " 176>    root\n...\n 181|            } else {\n 182|                format!(r\"Failed to open registry 'HKEY_CURRENT_USER\\{}'.\",parent_path).red()\n 183|            }\n 184|        })?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        tree.remove_tree(target_tree)
            .with_context(|| {
                if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "Failed to remove registry tree \'HKEY_LOCAL_MACHINE\\{0}\\{1}\'.",
                                parent_path,
                                target_tree,
                            ),
                        )
                    })
                } else {
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "Failed to remove registry tree \'HKEY_CURRENT_USER\\{0}\\{1}\'.",
                                parent_path,
                                target_tree,
                            ),
                        )
                    })
                }
            })
            .with_context(|| {
                let path = "src\\registry.rs";
                let line = 197usize;
                let col = 7usize;
                let expr = " 185>    tree.remove_tree(target_tree).with_context(|| {\n...\n 194|                parent_path, target_tree\n 195|            )\n 196|        }\n 197|    })?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        anyhow::Ok(())
    }
    pub fn delete_values(
        root: &windows_registry::Key,
        path: &str,
        keys: &[&str],
    ) -> anyhow::Result<()> {
        let tree = root
            .create(path)
            .with_context(|| {
                if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                    ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Failed to open registry \'HKEY_LOCAL_MACHINE\\{0}\'. You may need to run as administrator.",
                                    path,
                                ),
                            )
                        })
                        .red()
                } else {
                    ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Failed to open registry \'HKEY_CURRENT_USER\\{0}\'.",
                                    path,
                                ),
                            )
                        })
                        .red()
                }
            })
            .with_context(|| {
                let path = "src\\registry.rs";
                let line = 215usize;
                let col = 11usize;
                let expr = " 207>    root\n...\n 212|            } else {\n 213|                format!(r\"Failed to open registry 'HKEY_CURRENT_USER\\{}'.\",path).red()\n 214|            }\n 215|        })?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        for key in keys {
            tree.remove_value(key)
                .with_context(|| {
                    if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Failed to remove registry key \'HKEY_LOCAL_MACHINE\\{0}\\{1}\'.",
                                    path,
                                    key,
                                ),
                            )
                        })
                    } else {
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Failed to remove registry key \'HKEY_CURRENT_USER\\{0}\\{1}\'.",
                                    path,
                                    key,
                                ),
                            )
                        })
                    }
                })
                .with_context(|| {
                    let path = "src\\registry.rs";
                    let line = 229usize;
                    let col = 11usize;
                    let expr = " 217>    tree.remove_value(key).with_context(|| {\n...\n 226|                    path, key\n 227|                )\n 228|            }\n 229|        })?\n    |";
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                        )
                    })
                })?;
        }
        anyhow::Ok(())
    }
    pub fn check_deleted(
        root: &windows_registry::Key,
        path: &str,
        keys: &[&str],
    ) -> anyhow::Result<()> {
        let tree = root
            .create(path)
            .with_context(|| {
                if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                    ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Failed to open registry \'HKEY_LOCAL_MACHINE\\{0}\'. You may need to run as administrator.",
                                    path,
                                ),
                            )
                        })
                        .red()
                } else {
                    ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Failed to open registry \'HKEY_CURRENT_USER\\{0}\'.",
                                    path,
                                ),
                            )
                        })
                        .red()
                }
            })
            .with_context(|| {
                let path = "src\\registry.rs";
                let line = 248usize;
                let col = 11usize;
                let expr = " 240>    root\n...\n 245|            } else {\n 246|                format!(r\"Failed to open registry 'HKEY_CURRENT_USER\\{}'.\",path).red()\n 247|            }\n 248|        })?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        for key in keys {
            match tree.get_value(key) {
                Ok(v) => {
                    if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                        return anyhow::Result::Err(
                                anyhow::Error::msg(
                                    ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "Failed to delete registry \'HKEY_LOCAL_MACHINE\\{0}\\{1}\'.\\n Found: {2:?}",
                                                    path,
                                                    key,
                                                    v,
                                                ),
                                            )
                                        })
                                        .red(),
                                ),
                            )
                            .with_context(|| {
                                let path = "src\\registry.rs";
                                let line = 253usize;
                                let col = 21usize;
                                let expr = " 253>    return anyhow::Result::Err(anyhow::Error::msg(\n...\n 256|                            path, key, v\n 257|                        )\n 258|                        .red(),\n 259|                    ))\n    |";
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                    )
                                })
                            });
                    } else {
                        return anyhow::Result::Err(
                                anyhow::Error::msg(
                                    ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(
                                                    "Failed to read registry \'HKEY_CURRENT_USER\\{0}\\{1}\'.\\n Found: {2:?}",
                                                    path,
                                                    key,
                                                    v,
                                                ),
                                            )
                                        })
                                        .red(),
                                ),
                            )
                            .with_context(|| {
                                let path = "src\\registry.rs";
                                let line = 261usize;
                                let col = 21usize;
                                let expr = " 261>    return anyhow::Result::Err(anyhow::Error::msg(\n...\n 264|                            path, key, v\n 265|                        )\n 266|                        .red(),\n 267|                    ))\n    |";
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                    )
                                })
                            });
                    }
                }
                Err(e) => {
                    if e.code().0 == WIN32_ERROR_E_FILENOTFOUND.0
                    {} else if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw()
                    {
                        anyhow::Result::Err(anyhow::Error::from(e))
                            .with_context(|| {
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Failed to read registry \'HKEY_LOCAL_MACHINE\\{0}\\{1}\'.",
                                                path,
                                                key,
                                            ),
                                        )
                                    })
                                    .red()
                            })
                            .with_context(|| {
                                let path = "src\\registry.rs";
                                let line = 280usize;
                                let col = 23usize;
                                let expr = " 274>    anyhow::Result::Err(anyhow::Error::from(e)).with_context(|| {\n...\n 277|                            path, key\n 278|                        )\n 279|                        .red()\n 280|                    })?\n    |";
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                    )
                                })
                            })?;
                    } else {
                        anyhow::Result::Err(anyhow::Error::from(e))
                            .with_context(|| {
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Failed to read registry \'HKEY_CURRENT_USER\\{0}\\{1}\'.",
                                                path,
                                                key,
                                            ),
                                        )
                                    })
                                    .red()
                            })
                            .with_context(|| {
                                let path = "src\\registry.rs";
                                let line = 288usize;
                                let col = 23usize;
                                let expr = " 282>    anyhow::Result::Err(anyhow::Error::from(e)).with_context(|| {\n...\n 285|                            path, key\n 286|                        )\n 287|                        .red()\n 288|                    })?\n    |";
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                    )
                                })
                            })?;
                    }
                }
            }
        }
        anyhow::Ok(())
    }
}
mod runner {
    use anyhow::Context as _;
    use colored::Colorize;
    use hooq::hooq;
    use std::sync::{Arc, LazyLock, Mutex, OnceLock, RwLock};
    pub static MODE_NAMES: OnceLock<Vec<String>> = OnceLock::new();
    pub static MODE: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new("".into()));
    pub struct Runner {
        settings: crate::settings::Settings,
    }
    impl Runner {
        pub fn new(settings: crate::settings::Settings) -> Self {
            Self { settings }
        }
        #[inline]
        fn outset(
            &self,
            notification_action: Arc<
                Mutex<Option<crate::notification::NotificationAction>>,
            >,
        ) -> anyhow::Result<()> {
            if self.settings.select_mode_when_starts {
                crate::notification::mode_change_notify(
                        &self.settings.notification_method,
                        notification_action,
                    )
                    .with_context(|| {
                        let path = "src\\runner.rs";
                        let line = 30usize;
                        let col = 14usize;
                        let expr = "  27>    crate::notification::mode_change_notify(\n  28|                &self.settings.notification_method,\n  29|                notification_action,\n  30|            )?\n    |";
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                            )
                        })
                    })?;
            }
            if let Some(wait_time) = self
                .settings
                .wait_seconds_after_select_mode_notify_when_starts
            {
                std::thread::sleep(std::time::Duration::from_secs(wait_time));
            }
            Ok(())
        }
        pub fn run(self) -> anyhow::Result<()> {
            use crate::notification::{NotificationAction, battery_notify};
            let duration = std::time::Duration::from_secs(self.settings.check_interval);
            let mode_names = MODE_NAMES.get();
            MODE_NAMES
                .set(
                    self
                        .settings
                        .modes
                        .keys()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>(),
                )
                .map_err(|_| {
                    anyhow::Error::msg(
                        ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "MODE_NAMES is initialized. Found: {0:?} \nThis can not be happen.",
                                        mode_names,
                                    ),
                                )
                            })
                            .red(),
                    )
                })
                .with_context(|| {
                    let path = "src\\runner.rs";
                    let line = 61usize;
                    let col = 15usize;
                    let expr = "  45>    MODE_NAMES\n...\n  58|                    )\n  59|                    .red(),\n  60|                )\n  61|            })?\n    |";
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                        )
                    })
                })?;
            let mode_names = MODE_NAMES
                .get()
                .ok_or_else(|| {
                    anyhow::Error::msg(
                        "MODE_NAMES is not initilized. This can not happen.".red(),
                    )
                })
                .with_context(|| {
                    let path = "src\\runner.rs";
                    let line = 64usize;
                    let col = 11usize;
                    let expr = "  62>    MODE_NAMES.get().ok_or_else(|| {\n  63|            anyhow::Error::msg(\"MODE..pen.\".red())\n  64|        })?\n    |";
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                        )
                    })
                })?;
            let mode = &*MODE;
            match mode.write() {
                Ok(mut guard) => {
                    *guard = self.settings.initial_mode.clone();
                }
                Err(e) => {
                    return Err(
                            anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Failed to lock MODE. Unknown poison error!\n Error: {0:?}",
                                                e,
                                            ),
                                        )
                                    })
                                    .red(),
                            ),
                        )
                        .with_context(|| {
                            let path = "src\\runner.rs";
                            let line = 71usize;
                            let col = 17usize;
                            let expr = "  71>    return Err(anyhow::Error::msg(\n...\n  74|                        e\n  75|                    )\n  76|                    .red(),\n  77|                ))\n    |";
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                )
                            })
                        });
                }
            };
            let notification_method = &self.settings.notification_method;
            let notification_action: Arc<Mutex<Option<NotificationAction>>> = Arc::new(
                Mutex::new(None),
            );
            self.outset(Arc::clone(&notification_action))
                .with_context(|| {
                    let path = "src\\runner.rs";
                    let line = 83usize;
                    let col = 54usize;
                    let expr = "  83>    self.outset(Arc::clone(&notification_action))?\n    |";
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                        )
                    })
                })?;
            loop {
                let mut action_guard = match notification_action.lock() {
                    Ok(action_guard) => action_guard,
                    Err(e) => {
                        {
                            ::std::io::_eprint(
                                format_args!(
                                    "[{0}:{1}:{2}] notification_action.lock() in cli::Cli::run\n{3}\n",
                                    "src\\runner.rs",
                                    108u32 - 6,
                                    68,
                                    "poison error of Mutex of notification action.".red(),
                                ),
                            );
                        };
                        {
                            ::std::io::_eprint(
                                format_args!("ref: {0:?}\n", e.get_ref()),
                            );
                        };
                        e.into_inner()
                    }
                };
                if let Some(action) = &*action_guard {
                    match action {
                        NotificationAction::Silent5Mins => {
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "{0}\n",
                                        "Silent for 5 minutes action triggered.".yellow(),
                                    ),
                                );
                            };
                            std::thread::sleep(
                                std::time::Duration::from_secs(
                                    300u64.saturating_sub(self.settings.check_interval),
                                ),
                            );
                        }
                        NotificationAction::Silent10Mins => {
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "{0}\n",
                                        "Silent for 10 minutes action triggered.".yellow(),
                                    ),
                                );
                            };
                            std::thread::sleep(
                                std::time::Duration::from_secs(
                                    600u64.saturating_sub(self.settings.check_interval),
                                ),
                            );
                        }
                        NotificationAction::SilentSpecifiedMins(specified_mins) => {
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "{0}\n",
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!(
                                                        "Silent for {0} minutes action triggered.",
                                                        specified_mins,
                                                    ),
                                                )
                                            })
                                            .yellow(),
                                    ),
                                );
                            };
                            std::thread::sleep(
                                std::time::Duration::from_secs(
                                    specified_mins
                                        .saturating_mul(60)
                                        .saturating_sub(self.settings.check_interval),
                                ),
                            );
                        }
                        NotificationAction::RequireChangeMode => {
                            crate::notification::mode_change_notify(
                                    notification_method,
                                    Arc::clone(&notification_action),
                                )
                                .or_else(|e| {
                                    if self.settings.abort_on_error_except_initialize {
                                        Err(e)
                                            .with_context(|| {
                                                let path = "src\\runner.rs";
                                                let line = 146usize;
                                                let col = 26usize;
                                                let expr = " 143>    crate::notification::mode_change_notify(\n 144|                            notification_method,\n 145|                            Arc::clone(&notification_action),\n 146|                        )?\n    |";
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                                    )
                                                })
                                            })
                                    } else {
                                        let path = "src\\runner.rs";
                                        let line = 146usize;
                                        let col = 26usize;
                                        let expr = " 143>    crate::notification::mode_change_notify(\n 144|                            notification_method,\n 145|                            Arc::clone(&notification_action),\n 146|                        )?\n    |";
                                        {
                                            ::std::io::_eprint(
                                                format_args!("[{0}:{1}:{2}]\n{3}\n", path, line, col, expr),
                                            );
                                        };
                                        Ok(())
                                    }
                                })?;
                            if !self.settings.notify_battery_during_change_mode {
                                drop(action_guard);
                                std::thread::sleep(duration);
                                continue;
                            }
                        }
                        NotificationAction::ChangeMode(mode_to_change) => {
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "{0}\n",
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!(
                                                        "change mode to \"{0}\" action triggered.",
                                                        mode_to_change,
                                                    ),
                                                )
                                            })
                                            .yellow(),
                                    ),
                                );
                            };
                            if mode_names
                                .contains(&*mode.read().unwrap_or_else(|e| e.into_inner()))
                            {
                                match mode.write() {
                                    Ok(mut mode_guard) => {
                                        *mode_guard = mode_to_change.clone();
                                        drop(mode_guard);
                                    }
                                    Err(e) => {
                                        {
                                            ::std::io::_eprint(
                                                format_args!(
                                                    "[{0}:{1}:{2}] mode.lock() in cli::Cli::run\n{3}\n",
                                                    "src\\runner.rs",
                                                    170u32 - 8,
                                                    47,
                                                    "poison error of RwLock of mode.".red(),
                                                ),
                                            );
                                        };
                                        {
                                            ::std::io::_eprint(
                                                format_args!("ref: {0:?}\n", e.get_ref()),
                                            );
                                        };
                                        let mut mode_guard = e.into_inner();
                                        *mode_guard = mode_to_change.clone();
                                        drop(mode_guard);
                                    }
                                }
                            }
                        }
                        NotificationAction::Error(e) => {
                            Err(anyhow::Error::msg(e.to_string()))
                                .or_else(|e| {
                                    if self.settings.abort_on_error_except_initialize {
                                        Err(e)
                                            .with_context(|| {
                                                let path = "src\\runner.rs";
                                                let line = 183usize;
                                                let col = 63usize;
                                                let expr = " 183>    Err(anyhow::Error::msg(e.to_string()))?\n    |";
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                                    )
                                                })
                                            })
                                    } else {
                                        let path = "src\\runner.rs";
                                        let line = 183usize;
                                        let col = 63usize;
                                        let expr = " 183>    Err(anyhow::Error::msg(e.to_string()))?\n    |";
                                        {
                                            ::std::io::_eprint(
                                                format_args!("[{0}:{1}:{2}]\n{3}\n", path, line, col, expr),
                                            );
                                        };
                                        Ok(())
                                    }
                                })?;
                        }
                    }
                    *action_guard = None;
                }
                drop(action_guard);
                {
                    ::std::io::_print(
                        format_args!(
                            "mode: {0:?}\n",
                            mode.read().unwrap_or_else(|e| e.into_inner()),
                        ),
                    );
                };
                let battery_report = match crate::battery::battery_check() {
                    Ok(report) => report,
                    Err(e) => {
                        if self.settings.abort_on_error_except_initialize {
                            return Err(e)
                                .with_context(|| {
                                    let path = "src\\runner.rs";
                                    let line = 208usize;
                                    let col = 25usize;
                                    let expr = " 208>    return Err(e)\n    |";
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                        )
                                    })
                                });
                        } else {
                            {
                                ::std::io::_eprint(
                                    format_args!("Error on checking battery:\n{0:?}\n", e),
                                );
                            };
                            std::thread::sleep(duration);
                            continue;
                        }
                    }
                };
                match &battery_report {
                    tmp => {
                        {
                            ::std::io::_eprint(
                                format_args!(
                                    "[{0}:{1}:{2}] {3} = {4:#?}\n",
                                    "src\\runner.rs",
                                    216u32,
                                    13u32,
                                    "& battery_report",
                                    &&tmp as &dyn ::std::fmt::Debug,
                                ),
                            );
                        };
                        tmp
                    }
                };
                self.settings
                    .notifications
                    .iter()
                    .filter(|notification_setting| {
                        crate::notification::judge_notification(
                            notification_setting,
                            &battery_report,
                        )
                    })
                    .try_for_each(|notification_setting| {
                        battery_notify(
                            notification_method,
                            &battery_report,
                            &notification_setting.title,
                            &notification_setting.message,
                            Arc::clone(&notification_action),
                            &notification_setting.input_type,
                            mode_names,
                        )
                    })
                    .or_else(|e| {
                        if self.settings.abort_on_error_except_initialize {
                            Err(e)
                                .with_context(|| {
                                    let path = "src\\runner.rs";
                                    let line = 233usize;
                                    let col = 19usize;
                                    let expr = " 217>    self.settings\n...\n 230|                        &notification_setting.input_type,\n 231|                        mode_names,\n 232|                    )\n 233|                })?\n    |";
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                        )
                                    })
                                })
                        } else {
                            let path = "src\\runner.rs";
                            let line = 233usize;
                            let col = 19usize;
                            let expr = " 217>    self.settings\n...\n 230|                        &notification_setting.input_type,\n 231|                        mode_names,\n 232|                    )\n 233|                })?\n    |";
                            {
                                ::std::io::_eprint(
                                    format_args!("[{0}:{1}:{2}]\n{3}\n", path, line, col, expr),
                                );
                            };
                            Ok(())
                        }
                    })?;
                if let Some(mode_setting) = self
                    .settings
                    .modes
                    .get(&*mode.read().unwrap_or_else(|e| e.into_inner()))
                {
                    mode_setting
                        .notifications
                        .iter()
                        .filter(|notification_setting| {
                            crate::notification::judge_notification(
                                notification_setting,
                                &battery_report,
                            )
                        })
                        .try_for_each(|notification_setting| {
                            battery_notify(
                                notification_method,
                                &battery_report,
                                &notification_setting.title,
                                &notification_setting.message,
                                Arc::clone(&notification_action),
                                &notification_setting.input_type,
                                mode_names,
                            )
                        })
                        .or_else(|e| {
                            if self.settings.abort_on_error_except_initialize {
                                Err(e)
                                    .with_context(|| {
                                        let path = "src\\runner.rs";
                                        let line = 258usize;
                                        let col = 23usize;
                                        let expr = " 239>    mode_setting\n...\n 255|                            &notification_setting.input_type,\n 256|                            mode_names,\n 257|                        )\n 258|                    })?\n    |";
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                            )
                                        })
                                    })
                            } else {
                                let path = "src\\runner.rs";
                                let line = 258usize;
                                let col = 23usize;
                                let expr = " 239>    mode_setting\n...\n 255|                            &notification_setting.input_type,\n 256|                            mode_names,\n 257|                        )\n 258|                    })?\n    |";
                                {
                                    ::std::io::_eprint(
                                        format_args!("[{0}:{1}:{2}]\n{3}\n", path, line, col, expr),
                                    );
                                };
                                Ok(())
                            }
                        })?;
                }
                {
                    ::std::io::_print(format_args!("check battery and notifying\n"));
                };
                std::thread::sleep(duration);
            }
            #[allow(unreachable_code)] Ok(())
        }
    }
}
mod settings {
    use anyhow::Context as _;
    use colored::Colorize;
    use hooq::hooq;
    use rustc_hash::{FxBuildHasher, FxHashMap};
    type FxIndexMap<K, V> = indexmap::IndexMap<K, V, FxBuildHasher>;
    use crate::notification::NotificationInputType;
    use crate::notification::NotificationMethod;
    pub struct TOMLSettings {
        pub check_interval: u64,
        pub notification_method: Option<NotificationMethod>,
        pub mode_names: Option<Vec<String>>,
        pub initial_mode: Option<String>,
        pub abort_on_error_except_initialize: Option<bool>,
        pub notify_battery_during_change_mode: Option<bool>,
        pub select_mode_when_starts: Option<bool>,
        pub wait_seconds_after_select_mode_notify_when_starts: Option<u64>,
        pub notifications: Option<Vec<NotificationTOMLSetting>>,
        pub modes: Option<FxHashMap<String, ModeTOMLSetting>>,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TOMLSettings {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "TOMLSettings",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "check_interval",
                    &self.check_interval,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "notification_method",
                    &self.notification_method,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "mode_names",
                    &self.mode_names,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "initial_mode",
                    &self.initial_mode,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "abort_on_error_except_initialize",
                    &self.abort_on_error_except_initialize,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "notify_battery_during_change_mode",
                    &self.notify_battery_during_change_mode,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "select_mode_when_starts",
                    &self.select_mode_when_starts,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "wait_seconds_after_select_mode_notify_when_starts",
                    &self.wait_seconds_after_select_mode_notify_when_starts,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "notifications",
                    &self.notifications,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "modes",
                    &self.modes,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TOMLSettings {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            2u64 => _serde::__private228::Ok(__Field::__field2),
                            3u64 => _serde::__private228::Ok(__Field::__field3),
                            4u64 => _serde::__private228::Ok(__Field::__field4),
                            5u64 => _serde::__private228::Ok(__Field::__field5),
                            6u64 => _serde::__private228::Ok(__Field::__field6),
                            7u64 => _serde::__private228::Ok(__Field::__field7),
                            8u64 => _serde::__private228::Ok(__Field::__field8),
                            9u64 => _serde::__private228::Ok(__Field::__field9),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "check_interval" => {
                                _serde::__private228::Ok(__Field::__field0)
                            }
                            "notification_method" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            "mode_names" => _serde::__private228::Ok(__Field::__field2),
                            "initial_mode" => _serde::__private228::Ok(__Field::__field3),
                            "abort_on_error_except_initialize" => {
                                _serde::__private228::Ok(__Field::__field4)
                            }
                            "notify_battery_during_change_mode" => {
                                _serde::__private228::Ok(__Field::__field5)
                            }
                            "select_mode_when_starts" => {
                                _serde::__private228::Ok(__Field::__field6)
                            }
                            "wait_seconds_after_select_mode_notify_when_starts" => {
                                _serde::__private228::Ok(__Field::__field7)
                            }
                            "notifications" => {
                                _serde::__private228::Ok(__Field::__field8)
                            }
                            "modes" => _serde::__private228::Ok(__Field::__field9),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"check_interval" => {
                                _serde::__private228::Ok(__Field::__field0)
                            }
                            b"notification_method" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            b"mode_names" => _serde::__private228::Ok(__Field::__field2),
                            b"initial_mode" => {
                                _serde::__private228::Ok(__Field::__field3)
                            }
                            b"abort_on_error_except_initialize" => {
                                _serde::__private228::Ok(__Field::__field4)
                            }
                            b"notify_battery_during_change_mode" => {
                                _serde::__private228::Ok(__Field::__field5)
                            }
                            b"select_mode_when_starts" => {
                                _serde::__private228::Ok(__Field::__field6)
                            }
                            b"wait_seconds_after_select_mode_notify_when_starts" => {
                                _serde::__private228::Ok(__Field::__field7)
                            }
                            b"notifications" => {
                                _serde::__private228::Ok(__Field::__field8)
                            }
                            b"modes" => _serde::__private228::Ok(__Field::__field9),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<TOMLSettings>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TOMLSettings;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct TOMLSettings",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            u64,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct TOMLSettings with 10 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Option<NotificationMethod>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct TOMLSettings with 10 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Option<Vec<String>>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct TOMLSettings with 10 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct TOMLSettings with 10 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            Option<bool>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct TOMLSettings with 10 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match _serde::de::SeqAccess::next_element::<
                            Option<bool>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct TOMLSettings with 10 elements",
                                    ),
                                );
                            }
                        };
                        let __field6 = match _serde::de::SeqAccess::next_element::<
                            Option<bool>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct TOMLSettings with 10 elements",
                                    ),
                                );
                            }
                        };
                        let __field7 = match _serde::de::SeqAccess::next_element::<
                            Option<u64>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        7usize,
                                        &"struct TOMLSettings with 10 elements",
                                    ),
                                );
                            }
                        };
                        let __field8 = match _serde::de::SeqAccess::next_element::<
                            Option<Vec<NotificationTOMLSetting>>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        8usize,
                                        &"struct TOMLSettings with 10 elements",
                                    ),
                                );
                            }
                        };
                        let __field9 = match _serde::de::SeqAccess::next_element::<
                            Option<FxHashMap<String, ModeTOMLSetting>>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        9usize,
                                        &"struct TOMLSettings with 10 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(TOMLSettings {
                            check_interval: __field0,
                            notification_method: __field1,
                            mode_names: __field2,
                            initial_mode: __field3,
                            abort_on_error_except_initialize: __field4,
                            notify_battery_during_change_mode: __field5,
                            select_mode_when_starts: __field6,
                            wait_seconds_after_select_mode_notify_when_starts: __field7,
                            notifications: __field8,
                            modes: __field9,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<u64> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<
                            Option<NotificationMethod>,
                        > = _serde::__private228::None;
                        let mut __field2: _serde::__private228::Option<
                            Option<Vec<String>>,
                        > = _serde::__private228::None;
                        let mut __field3: _serde::__private228::Option<Option<String>> = _serde::__private228::None;
                        let mut __field4: _serde::__private228::Option<Option<bool>> = _serde::__private228::None;
                        let mut __field5: _serde::__private228::Option<Option<bool>> = _serde::__private228::None;
                        let mut __field6: _serde::__private228::Option<Option<bool>> = _serde::__private228::None;
                        let mut __field7: _serde::__private228::Option<Option<u64>> = _serde::__private228::None;
                        let mut __field8: _serde::__private228::Option<
                            Option<Vec<NotificationTOMLSetting>>,
                        > = _serde::__private228::None;
                        let mut __field9: _serde::__private228::Option<
                            Option<FxHashMap<String, ModeTOMLSetting>>,
                        > = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "check_interval",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<u64>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "notification_method",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<NotificationMethod>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private228::Option::is_some(&__field2) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "mode_names",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<Vec<String>>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private228::Option::is_some(&__field3) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "initial_mode",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private228::Option::is_some(&__field4) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "abort_on_error_except_initialize",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<bool>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private228::Option::is_some(&__field5) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "notify_battery_during_change_mode",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<bool>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private228::Option::is_some(&__field6) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "select_mode_when_starts",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<bool>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private228::Option::is_some(&__field7) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "wait_seconds_after_select_mode_notify_when_starts",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<u64>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::__private228::Option::is_some(&__field8) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "notifications",
                                            ),
                                        );
                                    }
                                    __field8 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<Vec<NotificationTOMLSetting>>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field9 => {
                                    if _serde::__private228::Option::is_some(&__field9) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("modes"),
                                        );
                                    }
                                    __field9 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<FxHashMap<String, ModeTOMLSetting>>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("check_interval")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field(
                                    "notification_method",
                                )?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private228::Some(__field2) => __field2,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("mode_names")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private228::Some(__field3) => __field3,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("initial_mode")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private228::Some(__field4) => __field4,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field(
                                    "abort_on_error_except_initialize",
                                )?
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private228::Some(__field5) => __field5,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field(
                                    "notify_battery_during_change_mode",
                                )?
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private228::Some(__field6) => __field6,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field(
                                    "select_mode_when_starts",
                                )?
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private228::Some(__field7) => __field7,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field(
                                    "wait_seconds_after_select_mode_notify_when_starts",
                                )?
                            }
                        };
                        let __field8 = match __field8 {
                            _serde::__private228::Some(__field8) => __field8,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("notifications")?
                            }
                        };
                        let __field9 = match __field9 {
                            _serde::__private228::Some(__field9) => __field9,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("modes")?
                            }
                        };
                        _serde::__private228::Ok(TOMLSettings {
                            check_interval: __field0,
                            notification_method: __field1,
                            mode_names: __field2,
                            initial_mode: __field3,
                            abort_on_error_except_initialize: __field4,
                            notify_battery_during_change_mode: __field5,
                            select_mode_when_starts: __field6,
                            wait_seconds_after_select_mode_notify_when_starts: __field7,
                            notifications: __field8,
                            modes: __field9,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "check_interval",
                    "notification_method",
                    "mode_names",
                    "initial_mode",
                    "abort_on_error_except_initialize",
                    "notify_battery_during_change_mode",
                    "select_mode_when_starts",
                    "wait_seconds_after_select_mode_notify_when_starts",
                    "notifications",
                    "modes",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TOMLSettings",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<TOMLSettings>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for TOMLSettings {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "check_interval",
                "notification_method",
                "mode_names",
                "initial_mode",
                "abort_on_error_except_initialize",
                "notify_battery_during_change_mode",
                "select_mode_when_starts",
                "wait_seconds_after_select_mode_notify_when_starts",
                "notifications",
                "modes",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.check_interval,
                &self.notification_method,
                &self.mode_names,
                &self.initial_mode,
                &self.abort_on_error_except_initialize,
                &self.notify_battery_during_change_mode,
                &self.select_mode_when_starts,
                &self.wait_seconds_after_select_mode_notify_when_starts,
                &self.notifications,
                &&self.modes,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "TOMLSettings",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TOMLSettings {
        #[inline]
        fn clone(&self) -> TOMLSettings {
            TOMLSettings {
                check_interval: ::core::clone::Clone::clone(&self.check_interval),
                notification_method: ::core::clone::Clone::clone(
                    &self.notification_method,
                ),
                mode_names: ::core::clone::Clone::clone(&self.mode_names),
                initial_mode: ::core::clone::Clone::clone(&self.initial_mode),
                abort_on_error_except_initialize: ::core::clone::Clone::clone(
                    &self.abort_on_error_except_initialize,
                ),
                notify_battery_during_change_mode: ::core::clone::Clone::clone(
                    &self.notify_battery_during_change_mode,
                ),
                select_mode_when_starts: ::core::clone::Clone::clone(
                    &self.select_mode_when_starts,
                ),
                wait_seconds_after_select_mode_notify_when_starts: ::core::clone::Clone::clone(
                    &self.wait_seconds_after_select_mode_notify_when_starts,
                ),
                notifications: ::core::clone::Clone::clone(&self.notifications),
                modes: ::core::clone::Clone::clone(&self.modes),
            }
        }
    }
    pub struct NotificationTOMLSetting {
        pub percentage: String,
        pub power_supply: String,
        pub title: String,
        pub message: String,
        pub input_type: Option<NotificationInputType>,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for NotificationTOMLSetting {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "NotificationTOMLSetting",
                    false as usize + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "percentage",
                    &self.percentage,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "power_supply",
                    &self.power_supply,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "title",
                    &self.title,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "message",
                    &self.message,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "input_type",
                    &self.input_type,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for NotificationTOMLSetting {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            2u64 => _serde::__private228::Ok(__Field::__field2),
                            3u64 => _serde::__private228::Ok(__Field::__field3),
                            4u64 => _serde::__private228::Ok(__Field::__field4),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "percentage" => _serde::__private228::Ok(__Field::__field0),
                            "power_supply" => _serde::__private228::Ok(__Field::__field1),
                            "title" => _serde::__private228::Ok(__Field::__field2),
                            "message" => _serde::__private228::Ok(__Field::__field3),
                            "input_type" => _serde::__private228::Ok(__Field::__field4),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"percentage" => _serde::__private228::Ok(__Field::__field0),
                            b"power_supply" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            b"title" => _serde::__private228::Ok(__Field::__field2),
                            b"message" => _serde::__private228::Ok(__Field::__field3),
                            b"input_type" => _serde::__private228::Ok(__Field::__field4),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<NotificationTOMLSetting>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = NotificationTOMLSetting;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct NotificationTOMLSetting",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct NotificationTOMLSetting with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct NotificationTOMLSetting with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct NotificationTOMLSetting with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct NotificationTOMLSetting with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            Option<NotificationInputType>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct NotificationTOMLSetting with 5 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(NotificationTOMLSetting {
                            percentage: __field0,
                            power_supply: __field1,
                            title: __field2,
                            message: __field3,
                            input_type: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<String> = _serde::__private228::None;
                        let mut __field2: _serde::__private228::Option<String> = _serde::__private228::None;
                        let mut __field3: _serde::__private228::Option<String> = _serde::__private228::None;
                        let mut __field4: _serde::__private228::Option<
                            Option<NotificationInputType>,
                        > = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "percentage",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "power_supply",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private228::Option::is_some(&__field2) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("title"),
                                        );
                                    }
                                    __field2 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private228::Option::is_some(&__field3) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "message",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private228::Option::is_some(&__field4) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "input_type",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<NotificationInputType>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("percentage")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("power_supply")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private228::Some(__field2) => __field2,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("title")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private228::Some(__field3) => __field3,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("message")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private228::Some(__field4) => __field4,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("input_type")?
                            }
                        };
                        _serde::__private228::Ok(NotificationTOMLSetting {
                            percentage: __field0,
                            power_supply: __field1,
                            title: __field2,
                            message: __field3,
                            input_type: __field4,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "percentage",
                    "power_supply",
                    "title",
                    "message",
                    "input_type",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "NotificationTOMLSetting",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<
                            NotificationTOMLSetting,
                        >,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for NotificationTOMLSetting {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "NotificationTOMLSetting",
                "percentage",
                &self.percentage,
                "power_supply",
                &self.power_supply,
                "title",
                &self.title,
                "message",
                &self.message,
                "input_type",
                &&self.input_type,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for NotificationTOMLSetting {
        #[inline]
        fn clone(&self) -> NotificationTOMLSetting {
            NotificationTOMLSetting {
                percentage: ::core::clone::Clone::clone(&self.percentage),
                power_supply: ::core::clone::Clone::clone(&self.power_supply),
                title: ::core::clone::Clone::clone(&self.title),
                message: ::core::clone::Clone::clone(&self.message),
                input_type: ::core::clone::Clone::clone(&self.input_type),
            }
        }
    }
    pub struct ModeTOMLSetting {
        pub notifications: Vec<NotificationTOMLSetting>,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ModeTOMLSetting {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "ModeTOMLSetting",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "notifications",
                    &self.notifications,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ModeTOMLSetting {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "notifications" => {
                                _serde::__private228::Ok(__Field::__field0)
                            }
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"notifications" => {
                                _serde::__private228::Ok(__Field::__field0)
                            }
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<ModeTOMLSetting>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ModeTOMLSetting;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct ModeTOMLSetting",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Vec<NotificationTOMLSetting>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct ModeTOMLSetting with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(ModeTOMLSetting {
                            notifications: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<
                            Vec<NotificationTOMLSetting>,
                        > = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "notifications",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<NotificationTOMLSetting>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("notifications")?
                            }
                        };
                        _serde::__private228::Ok(ModeTOMLSetting {
                            notifications: __field0,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["notifications"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ModeTOMLSetting",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<ModeTOMLSetting>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for ModeTOMLSetting {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ModeTOMLSetting",
                "notifications",
                &&self.notifications,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ModeTOMLSetting {
        #[inline]
        fn clone(&self) -> ModeTOMLSetting {
            ModeTOMLSetting {
                notifications: ::core::clone::Clone::clone(&self.notifications),
            }
        }
    }
    pub struct Settings {
        pub check_interval: u64,
        pub notification_method: NotificationMethod,
        pub initial_mode: String,
        pub abort_on_error_except_initialize: bool,
        pub notify_battery_during_change_mode: bool,
        pub select_mode_when_starts: bool,
        pub wait_seconds_after_select_mode_notify_when_starts: Option<u64>,
        pub notifications: Vec<NotificationSetting>,
        pub modes: FxIndexMap<String, ModeSetting>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Settings {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "check_interval",
                "notification_method",
                "initial_mode",
                "abort_on_error_except_initialize",
                "notify_battery_during_change_mode",
                "select_mode_when_starts",
                "wait_seconds_after_select_mode_notify_when_starts",
                "notifications",
                "modes",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.check_interval,
                &self.notification_method,
                &self.initial_mode,
                &self.abort_on_error_except_initialize,
                &self.notify_battery_during_change_mode,
                &self.select_mode_when_starts,
                &self.wait_seconds_after_select_mode_notify_when_starts,
                &self.notifications,
                &&self.modes,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "Settings",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Settings {
        #[inline]
        fn clone(&self) -> Settings {
            Settings {
                check_interval: ::core::clone::Clone::clone(&self.check_interval),
                notification_method: ::core::clone::Clone::clone(
                    &self.notification_method,
                ),
                initial_mode: ::core::clone::Clone::clone(&self.initial_mode),
                abort_on_error_except_initialize: ::core::clone::Clone::clone(
                    &self.abort_on_error_except_initialize,
                ),
                notify_battery_during_change_mode: ::core::clone::Clone::clone(
                    &self.notify_battery_during_change_mode,
                ),
                select_mode_when_starts: ::core::clone::Clone::clone(
                    &self.select_mode_when_starts,
                ),
                wait_seconds_after_select_mode_notify_when_starts: ::core::clone::Clone::clone(
                    &self.wait_seconds_after_select_mode_notify_when_starts,
                ),
                notifications: ::core::clone::Clone::clone(&self.notifications),
                modes: ::core::clone::Clone::clone(&self.modes),
            }
        }
    }
    pub struct NotificationSetting {
        pub percentage_int: u32,
        pub percentage_symbol: PercentageSymbol,
        pub power_supply: crate::battery::PowerSupply,
        pub title: String,
        pub message: String,
        pub input_type: NotificationInputType,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for NotificationSetting {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "percentage_int",
                "percentage_symbol",
                "power_supply",
                "title",
                "message",
                "input_type",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.percentage_int,
                &self.percentage_symbol,
                &self.power_supply,
                &self.title,
                &self.message,
                &&self.input_type,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "NotificationSetting",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for NotificationSetting {
        #[inline]
        fn clone(&self) -> NotificationSetting {
            NotificationSetting {
                percentage_int: ::core::clone::Clone::clone(&self.percentage_int),
                percentage_symbol: ::core::clone::Clone::clone(&self.percentage_symbol),
                power_supply: ::core::clone::Clone::clone(&self.power_supply),
                title: ::core::clone::Clone::clone(&self.title),
                message: ::core::clone::Clone::clone(&self.message),
                input_type: ::core::clone::Clone::clone(&self.input_type),
            }
        }
    }
    pub enum PercentageSymbol {
        Excess,
        Under,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for PercentageSymbol {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    PercentageSymbol::Excess => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "PercentageSymbol",
                            0u32,
                            "Excess",
                        )
                    }
                    PercentageSymbol::Under => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "PercentageSymbol",
                            1u32,
                            "Under",
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for PercentageSymbol {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            _ => {
                                _serde::__private228::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 2",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Excess" => _serde::__private228::Ok(__Field::__field0),
                            "Under" => _serde::__private228::Ok(__Field::__field1),
                            _ => {
                                _serde::__private228::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Excess" => _serde::__private228::Ok(__Field::__field0),
                            b"Under" => _serde::__private228::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private228::from_utf8_lossy(
                                    __value,
                                );
                                _serde::__private228::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<PercentageSymbol>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = PercentageSymbol;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "enum PercentageSymbol",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private228::Ok(PercentageSymbol::Excess)
                            }
                            (__Field::__field1, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private228::Ok(PercentageSymbol::Under)
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["Excess", "Under"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "PercentageSymbol",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<PercentageSymbol>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for PercentageSymbol {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    PercentageSymbol::Excess => "Excess",
                    PercentageSymbol::Under => "Under",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PercentageSymbol {
        #[inline]
        fn clone(&self) -> PercentageSymbol {
            match self {
                PercentageSymbol::Excess => PercentageSymbol::Excess,
                PercentageSymbol::Under => PercentageSymbol::Under,
            }
        }
    }
    pub struct ModeSetting {
        pub notifications: Vec<NotificationSetting>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ModeSetting {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ModeSetting",
                "notifications",
                &&self.notifications,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ModeSetting {
        #[inline]
        fn clone(&self) -> ModeSetting {
            ModeSetting {
                notifications: ::core::clone::Clone::clone(&self.notifications),
            }
        }
    }
    impl TryFrom<TOMLSettings> for Settings {
        type Error = anyhow::Error;
        fn try_from(toml_settings: TOMLSettings) -> anyhow::Result<Self> {
            let mut settings = Settings {
                check_interval: toml_settings.check_interval,
                notification_method: toml_settings
                    .notification_method
                    .unwrap_or_default(),
                initial_mode: toml_settings
                    .initial_mode
                    .map_or_else(
                        String::default,
                        |initial_mode| {
                            if let Some(modes) = toml_settings.modes.as_ref() {
                                if modes.keys().any(|key| key == &initial_mode) {
                                    initial_mode
                                } else {
                                    String::default()
                                }
                            } else {
                                String::default()
                            }
                        },
                    ),
                abort_on_error_except_initialize: toml_settings
                    .abort_on_error_except_initialize
                    .unwrap_or(false),
                notify_battery_during_change_mode: toml_settings
                    .notify_battery_during_change_mode
                    .unwrap_or(false),
                select_mode_when_starts: toml_settings
                    .select_mode_when_starts
                    .unwrap_or(true),
                wait_seconds_after_select_mode_notify_when_starts: match toml_settings
                    .wait_seconds_after_select_mode_notify_when_starts
                {
                    Some(0) => None,
                    Some(seconds) => Some(seconds),
                    None => Some(10),
                },
                notifications: Vec::with_capacity(
                    toml_settings
                        .notifications
                        .as_ref()
                        .map_or(0usize, |notifications| notifications.len()),
                ),
                modes: FxIndexMap::with_capacity_and_hasher(
                    toml_settings.modes.as_ref().map_or(0usize, |modes| modes.len()),
                    Default::default(),
                ),
            };
            for notification_toml_setting in toml_settings
                .notifications
                .unwrap_or_default()
            {
                settings
                    .notifications
                    .push(
                        notification_toml_setting
                            .try_into()
                            .with_context(|| {
                                let path = "src\\settings.rs";
                                let line = 125usize;
                                let col = 59usize;
                                let expr = " 125>    notification_toml_setting.try_into()?\n    |";
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                    )
                                })
                            })?,
                    );
            }
            if let Some(modes) = toml_settings.modes {
                if let Some(mode_names) = toml_settings.mode_names.as_ref() {
                    for mode_name in mode_names {
                        if let Some(mode_toml_setting) = modes.get(mode_name) {
                            settings
                                .modes
                                .insert(
                                    mode_name.clone(),
                                    mode_toml_setting
                                        .to_owned()
                                        .try_into()
                                        .with_context(|| {
                                            let path = "src\\settings.rs";
                                            let line = 133usize;
                                            let col = 95usize;
                                            let expr = " 133>    mode_toml_setting.to_owned().try_into()?\n    |";
                                            ::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                                )
                                            })
                                        })?,
                                );
                        }
                    }
                }
                for (mode, mode_toml_setting) in modes
                    .iter()
                    .filter(|(key, _)| {
                        if let Some(mode_names) = toml_settings.mode_names.as_ref() {
                            !mode_names.contains(key)
                        } else {
                            true
                        }
                    })
                {
                    settings
                        .modes
                        .insert(
                            mode.clone(),
                            mode_toml_setting
                                .to_owned()
                                .try_into()
                                .with_context(|| {
                                    let path = "src\\settings.rs";
                                    let line = 146usize;
                                    let col = 82usize;
                                    let expr = " 146>    mode_toml_setting.to_owned().try_into()?\n    |";
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                        )
                                    })
                                })?,
                        );
                }
            }
            match &settings {
                tmp => {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "[{0}:{1}:{2}] {3} = {4:#?}\n",
                                "src\\settings.rs",
                                150u32,
                                9u32,
                                "& settings",
                                &&tmp as &dyn ::std::fmt::Debug,
                            ),
                        );
                    };
                    tmp
                }
            };
            Ok(settings)
        }
    }
    impl TryFrom<ModeTOMLSetting> for ModeSetting {
        type Error = anyhow::Error;
        fn try_from(mode_toml_setting: ModeTOMLSetting) -> anyhow::Result<Self> {
            let mut mode_settings = Self {
                notifications: Vec::with_capacity(mode_toml_setting.notifications.len()),
            };
            for notification_setting in mode_toml_setting.notifications {
                mode_settings
                    .notifications
                    .push(
                        notification_setting
                            .try_into()
                            .with_context(|| {
                                let path = "src\\settings.rs";
                                let line = 166usize;
                                let col = 54usize;
                                let expr = " 166>    notification_setting.try_into()?\n    |";
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                    )
                                })
                            })?,
                    );
            }
            Ok(mode_settings)
        }
    }
    impl TryFrom<NotificationTOMLSetting> for NotificationSetting {
        type Error = anyhow::Error;
        fn try_from(
            notification_toml_setting: NotificationTOMLSetting,
        ) -> anyhow::Result<Self> {
            let percentage = notification_toml_setting.percentage;
            let Some(percentage_symbol) = percentage.chars().last() else {
                return Err(
                        anyhow::Error::msg(
                            ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "percentage may be empty. found:\'{0}\'.",
                                            &percentage,
                                        ),
                                    )
                                })
                                .red(),
                        ),
                    )
                    .with_context(|| {
                        let path = "src\\settings.rs";
                        let line = 179usize;
                        let col = 13usize;
                        let expr = " 179>    return Err(anyhow::Error::msg(\n 180|                format!(\"percentage may be empty. found:'{}'.\", &percentage).red(),\n 181|            ))\n    |";
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                            )
                        })
                    });
            };
            let percentage_symbol = match percentage_symbol {
                '+' => PercentageSymbol::Excess,
                '-' => PercentageSymbol::Under,
                _ => {
                    return Err(
                            anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Failed to interpret \'{0}\' as percentage symbol.",
                                                &percentage_symbol,
                                            ),
                                        )
                                    })
                                    .red(),
                            ),
                        )
                        .with_context(|| {
                            let path = "src\\settings.rs";
                            let line = 187usize;
                            let col = 17usize;
                            let expr = " 187>    return Err(anyhow::Error::msg(\n...\n 190|                        &percentage_symbol\n 191|                    )\n 192|                    .red(),\n 193|                ))\n    |";
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                )
                            })
                        });
                }
            };
            let percentage_int: u32 = percentage[0..percentage.len() - 1]
                .parse()
                .with_context(|| {
                    ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Failed to interpret \'{0}\' as percentage value.",
                                    &percentage,
                                ),
                            )
                        })
                        .red()
                })
                .with_context(|| {
                    let path = "src\\settings.rs";
                    let line = 201usize;
                    let col = 19usize;
                    let expr = " 197>    percentage[0..percentage.len() - 1]\n 198|                .parse()\n 199|                .with_context(|| {\n 200|                    format!(\"Failed to interpret '{}' as percentage value.\", &percentage).red()\n 201|                })?\n    |";
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                        )
                    })
                })?;
            let power_supply: crate::battery::PowerSupply = match notification_toml_setting
                .power_supply
                .as_str()
            {
                "Adequate" => crate::battery::PowerSupply::Adequate,
                "InAdequate" => crate::battery::PowerSupply::InAdequate,
                "None" => crate::battery::PowerSupply::None,
                _ => {
                    return Err(
                            anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Failed to interpret power_supply:\'{0}\'. Use \"Adequate\" , \"InAdequate\" or \"None\".",
                                                &notification_toml_setting.power_supply,
                                            ),
                                        )
                                    })
                                    .red(),
                            ),
                        )
                        .with_context(|| {
                            let path = "src\\settings.rs";
                            let line = 210usize;
                            let col = 17usize;
                            let expr = " 210>    return Err(anyhow::Error::msg(format!(\n 211|                    r#\"Failed to interpret power_supply:'{}'. Use \"Adequate\" , \"InAdequate\" or \"None\".\"#,\n 212|                    &notification_toml_setting.power_supply\n 213|                ).red()))\n    |";
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                )
                            })
                        });
                }
            };
            Ok(NotificationSetting {
                percentage_int,
                percentage_symbol,
                power_supply,
                title: notification_toml_setting.title,
                message: notification_toml_setting.message,
                input_type: notification_toml_setting.input_type.unwrap_or_default(),
            })
        }
    }
}
mod startup {
    use anyhow::Context as _;
    use colored::Colorize;
    use hooq::hooq;
    #[allow(unused)]
    use crate::registry::{
        CURRENT_USER, LOCAL_MACHINE, RegistryValue, check_deleted, check_registered,
        delete_values, register,
    };
    const REG_STARTUP_KEY: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";
    const REG_STARTUP_NAME: &str = "yy-tromb.yy-battery-notifier-rs";
    const REG_STARTUP_APPROVED_KEY: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run";
    const REG_STARTUP_APPROVED_VALUE: [u8; 12] = [
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    pub fn register_cli(
        toml_settings_path: Option<String>,
        input_mode: bool,
        default_settings: bool,
    ) -> anyhow::Result<()> {
        let toml_settings_path: String = match toml_settings_path {
            Some(toml_settings_path) => toml_settings_path,
            None => {
                if input_mode && default_settings {
                    return anyhow::Result::Err(
                            anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "This session is {0} and {1}.\n{2}. If you want to enable input mode, you can use --input",
                                                "input mode".bold(),
                                                "using default_settings".bold(),
                                                "Use one option".bold(),
                                            ),
                                        )
                                    })
                                    .red(),
                            ),
                        )
                        .with_context(|| {
                            let path = "src\\startup.rs";
                            let line = 29usize;
                            let col = 17usize;
                            let expr = "  29>    return anyhow::Result::Err(anyhow::Error::msg(\n...\n  36|                        \"Use one option\".bold()\n  37|                    )\n  38|                    .red(),\n  39|                ))\n    |";
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                )
                            })
                        });
                } else if input_mode && (!default_settings) {
                    let mut toml_settings_path_input = String::with_capacity(256);
                    loop {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Input or Paste the path to settings.toml\n(like C:\\Users\\Documents\\yy-battery-notifier-rs\\settings.toml):\n",
                                ),
                            );
                        };
                        match std::io::stdin().read_line(&mut toml_settings_path_input) {
                            Ok(len) => {
                                if len == 0 || toml_settings_path_input.trim().is_empty() {
                                    {
                                        ::std::io::_eprint(
                                            format_args!("{0}\n", "Given is empty.".red()),
                                        );
                                    };
                                } else {
                                    break;
                                }
                            }
                            Err(e) => {
                                ::std::io::_eprint(format_args!("{0}\n", e));
                            }
                        };
                    }
                    toml_settings_path_input
                } else if (!input_mode) & default_settings {
                    let default_toml_settings_path = std::env::current_exe()
                        .with_context(|| "Failed to get current exe".red())
                        .with_context(|| {
                            let path = "src\\startup.rs";
                            let line = 60usize;
                            let col = 72usize;
                            let expr = "  59>    std::env::current_exe()\n  60|                    .with_context(|| \"Fail.. exe\".red())?\n    |";
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                )
                            })
                        })?
                        .with_file_name("default_settings.toml");
                    let default_toml_settings_path = default_toml_settings_path
                        .to_str()
                        .with_context(|| {
                            "path to current exe is empty. Unknown error may occured."
                                .red()
                        })
                        .with_context(|| {
                            let path = "src\\startup.rs";
                            let line = 65usize;
                            let col = 23usize;
                            let expr = "  63>    default_toml_settings_path.to_str().with_context(|| {\n  64|                        \"path..red.\".red()\n  65|                    })?\n    |";
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                )
                            })
                        })?;
                    default_toml_settings_path.to_string()
                } else {
                    return anyhow::Result::Err(
                            anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "This session is {0} nor {1}.\n{2}. If you want to enable input mode, you can use --input",
                                                "input mode".bold(),
                                                "using default_settings".bold(),
                                                "Use one option".bold(),
                                            ),
                                        )
                                    })
                                    .red(),
                            ),
                        )
                        .with_context(|| {
                            let path = "src\\startup.rs";
                            let line = 68usize;
                            let col = 17usize;
                            let expr = "  68>    return anyhow::Result::Err(anyhow::Error::msg(\n...\n  75|                        \"Use one option\".bold()\n  76|                    )\n  77|                    .red(),\n  78|                ))\n    |";
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                )
                            })
                        });
                }
            }
        };
        let toml_settings_path = toml_settings_path.trim();
        let mut toml_settings_path_iter = toml_settings_path.chars();
        let start_index = match toml_settings_path_iter.next() {
            Some('"') | Some('\'') | Some('`') => 1,
            Some(_) => 0,
            None => ::core::panicking::panic("internal error: entered unreachable code"),
        };
        let end_index = match toml_settings_path_iter.nth_back(0) {
            Some('"') | Some('\'') | Some('`') => toml_settings_path.len() - 1,
            Some(_) => toml_settings_path.len(),
            None => ::core::panicking::panic("internal error: entered unreachable code"),
        };
        let toml_settings_path_absolute = std::fs::canonicalize(
                &toml_settings_path[start_index..end_index],
            )
            .with_context(|| {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "Failed to canonicalize path: {0}",
                                toml_settings_path,
                            ),
                        )
                    })
                    .red()
            })
            .with_context(|| {
                let path = "src\\startup.rs";
                let line = 96usize;
                let col = 11usize;
                let expr = "  94>    std::fs::canonicalize(&toml_settings_path[start_index..end_index]).with_context(|| {\n  95|            format!(\"Failed to canonicalize path: {}\", toml_settings_path).red()\n  96|        })?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        let toml_settings_path_absolute = toml_settings_path_absolute
            .to_str()
            .with_context(|| "Failed to get absolute path string".red())
            .with_context(|| {
                let path = "src\\startup.rs";
                let line = 99usize;
                let col = 69usize;
                let expr = "  97>    toml_settings_path_absolute\n  98|        .to_str()\n  99|        .with_context(|| \"Fail..ring\".red())?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        {
            ::std::io::_print(
                format_args!(
                    "Now start register. settings.toml file: \'{0}\'\n",
                    toml_settings_path_absolute,
                ),
            );
        };
        register_and_check_startup(toml_settings_path_absolute.to_string())
            .with_context(|| {
                let path = "src\\startup.rs";
                let line = 104usize;
                let col = 72usize;
                let expr = " 104>    register_and_check_startup(toml_settings_path_absolute.to_string())?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        anyhow::Ok(())
    }
    #[inline]
    fn register_and_check_startup(toml_settings_path: String) -> anyhow::Result<()> {
        match &toml_settings_path {
            tmp => {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3} = {4:#?}\n",
                            "src\\startup.rs",
                            125u32,
                            5u32,
                            "& toml_settings_path",
                            &&tmp as &dyn ::std::fmt::Debug,
                        ),
                    );
                };
                tmp
            }
        };
        let current_exe = std::env::current_exe()
            .with_context(|| "Failed to get current exe".red())
            .with_context(|| {
                let path = "src\\startup.rs";
                let line = 129usize;
                let col = 60usize;
                let expr = " 128>    std::env::current_exe()\n 129|        .with_context(|| \"Fail.. exe\".red())?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?
            .with_file_name("yy-battery-notifier-rs_gui.exe");
        let current_exe = current_exe
            .to_str()
            .with_context(|| {
                "path to current exe is empty. Unknown error may occured.".red()
            })
            .with_context(|| {
                let path = "src\\startup.rs";
                let line = 133usize;
                let col = 91usize;
                let expr = " 131>    current_exe\n 132|        .to_str()\n 133|        .with_context(|| \"path..red.\".red())?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        let run_cmd = ::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "\"{0}\" --msgbox -s \"{1}\"",
                    current_exe,
                    toml_settings_path,
                ),
            )
        });
        let keys_and_values = <[_]>::into_vec(
            ::alloc::boxed::box_new([(REG_STARTUP_NAME, RegistryValue::String(run_cmd))]),
        );
        register(CURRENT_USER, REG_STARTUP_KEY, &keys_and_values)
            .with_context(|| {
                let path = "src\\startup.rs";
                let line = 138usize;
                let col = 62usize;
                let expr = " 138>    register(CURRENT_USER, REG_STARTUP_KEY, &keys_and_values)?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        check_registered(CURRENT_USER, REG_STARTUP_KEY, &keys_and_values)
            .with_context(|| {
                let path = "src\\startup.rs";
                let line = 141usize;
                let col = 70usize;
                let expr = " 141>    check_registered(CURRENT_USER, REG_STARTUP_KEY, &keys_and_values)?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        let keys_and_values = <[_]>::into_vec(
            ::alloc::boxed::box_new([
                (REG_STARTUP_NAME, RegistryValue::Bytes(&REG_STARTUP_APPROVED_VALUE)),
            ]),
        );
        register(CURRENT_USER, REG_STARTUP_APPROVED_KEY, &keys_and_values)
            .with_context(|| {
                let path = "src\\startup.rs";
                let line = 148usize;
                let col = 71usize;
                let expr = " 148>    register(CURRENT_USER, REG_STARTUP_APPROVED_KEY, &keys_and_values)?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        check_registered(CURRENT_USER, REG_STARTUP_APPROVED_KEY, &keys_and_values)
            .with_context(|| {
                let path = "src\\startup.rs";
                let line = 151usize;
                let col = 79usize;
                let expr = " 151>    check_registered(CURRENT_USER, REG_STARTUP_APPROVED_KEY, &keys_and_values)?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        {
            ::std::io::_print(
                format_args!("{0}\n", "register sucuessed!".green().on_black()),
            );
        };
        anyhow::Ok(())
    }
    #[inline]
    pub fn delete_and_check_startup() -> anyhow::Result<()> {
        let keys = <[_]>::into_vec(::alloc::boxed::box_new([REG_STARTUP_NAME]));
        delete_values(CURRENT_USER, REG_STARTUP_KEY, &keys)
            .with_context(|| {
                let path = "src\\startup.rs";
                let line = 161usize;
                let col = 56usize;
                let expr = " 161>    delete_values(CURRENT_USER, REG_STARTUP_KEY, &keys)?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        check_deleted(CURRENT_USER, REG_STARTUP_KEY, &keys)
            .with_context(|| {
                let path = "src\\startup.rs";
                let line = 164usize;
                let col = 56usize;
                let expr = " 164>    check_deleted(CURRENT_USER, REG_STARTUP_KEY, &keys)?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        delete_values(CURRENT_USER, REG_STARTUP_APPROVED_KEY, &keys)
            .with_context(|| {
                let path = "src\\startup.rs";
                let line = 168usize;
                let col = 65usize;
                let expr = " 168>    delete_values(CURRENT_USER, REG_STARTUP_APPROVED_KEY, &keys)?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        check_deleted(CURRENT_USER, REG_STARTUP_APPROVED_KEY, &keys)
            .with_context(|| {
                let path = "src\\startup.rs";
                let line = 171usize;
                let col = 65usize;
                let expr = " 171>    check_deleted(CURRENT_USER, REG_STARTUP_APPROVED_KEY, &keys)?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })?;
        {
            ::std::io::_print(
                format_args!("{0}\n", "delete sucuessed!".green().on_black()),
            );
        };
        anyhow::Ok(())
    }
}
#[allow(unused_imports)]
use registry::{CURRENT_USER, LOCAL_MACHINE};
#[command(
    name = env!("CARGO_PKG_NAME"),
    version,
    about = format!(
        "\x1b[1m\
{} version {}\x1b[m
{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION")
    ),
    author = "yy-tromb",
    long_about = None,
    disable_help_subcommand(true)
)]
struct AppArgs {
    #[arg(
        short = 's',
        long = "settings",
        value_name = "path to settings.toml",
        default_value_t = String::from(".\\settings.toml")
    )]
    toml_settings_path: String,
    #[arg(
        short = 'd',
        long = "default_settings",
        value_name = "use default_settings.toml",
        required = false
    )]
    default_settings: bool,
    #[arg(
        long = "msgbox",
        value_name = "enable message box",
        required = false,
        global = true
    )]
    msgbox: bool,
    #[command(subcommand)]
    subcommands: Option<SubCommand>,
}
#[automatically_derived]
#[allow(unused_qualifications, clippy::redundant_locals)]
impl clap::Parser for AppArgs {}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::CommandFactory for AppArgs {
    fn command<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("yy-battery-notifier-rs");
        <Self as clap::Args>::augment_args(__clap_app)
    }
    fn command_for_update<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("yy-battery-notifier-rs");
        <Self as clap::Args>::augment_args_for_update(__clap_app)
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::FromArgMatches for AppArgs {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = AppArgs {
            toml_settings_path: __clap_arg_matches
                .remove_one::<String>("toml_settings_path")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "the following required argument was not provided: toml_settings_path",
                ))?,
            default_settings: __clap_arg_matches
                .remove_one::<bool>("default_settings")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "the following required argument was not provided: default_settings",
                ))?,
            msgbox: __clap_arg_matches
                .remove_one::<bool>("msgbox")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "the following required argument was not provided: msgbox",
                ))?,
            subcommands: {
                if __clap_arg_matches
                    .subcommand_name()
                    .map(<SubCommand as clap::Subcommand>::has_subcommand)
                    .unwrap_or(false)
                {
                    Some(
                        <SubCommand as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?,
                    )
                } else {
                    None
                }
            },
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if __clap_arg_matches.contains_id("toml_settings_path") {
            #[allow(non_snake_case)]
            let toml_settings_path = &mut self.toml_settings_path;
            *toml_settings_path = __clap_arg_matches
                .remove_one::<String>("toml_settings_path")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "the following required argument was not provided: toml_settings_path",
                ))?;
        }
        if __clap_arg_matches.contains_id("default_settings") {
            #[allow(non_snake_case)]
            let default_settings = &mut self.default_settings;
            *default_settings = __clap_arg_matches
                .remove_one::<bool>("default_settings")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "the following required argument was not provided: default_settings",
                ))?;
        }
        if __clap_arg_matches.contains_id("msgbox") {
            #[allow(non_snake_case)]
            let msgbox = &mut self.msgbox;
            *msgbox = __clap_arg_matches
                .remove_one::<bool>("msgbox")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "the following required argument was not provided: msgbox",
                ))?;
        }
        {
            #[allow(non_snake_case)]
            let subcommands = &mut self.subcommands;
            if let Some(subcommands) = subcommands.as_mut() {
                <SubCommand as clap::FromArgMatches>::update_from_arg_matches_mut(
                    subcommands,
                    __clap_arg_matches,
                )?;
            } else {
                *subcommands = Some(
                    <SubCommand as clap::FromArgMatches>::from_arg_matches_mut(
                        __clap_arg_matches,
                    )?,
                );
            }
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::Args for AppArgs {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("AppArgs"))
    }
    fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("AppArgs")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 3usize] = [
                                clap::Id::from("toml_settings_path"),
                                clap::Id::from("default_settings"),
                                clap::Id::from("msgbox"),
                            ];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("toml_settings_path")
                        .value_name("TOML_SETTINGS_PATH")
                        .required(false && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::impl_prelude::*;
                            let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                String,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .short('s')
                        .long("settings")
                        .value_name("path to settings.toml")
                        .default_value({
                            static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                            let s = DEFAULT_VALUE
                                .get_or_init(|| {
                                    let val: String = String::from(".\\settings.toml");
                                    ::std::string::ToString::to_string(&val)
                                });
                            let s: &'static str = &*s;
                            s
                        });
                    let arg = arg;
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("default_settings")
                        .value_name("DEFAULT_SETTINGS")
                        .required(true && clap::ArgAction::SetTrue.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::impl_prelude::*;
                            let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                bool,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::SetTrue);
                    let arg = arg
                        .short('d')
                        .long("default_settings")
                        .value_name("use default_settings.toml")
                        .required(false);
                    let arg = arg;
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("msgbox")
                        .value_name("MSGBOX")
                        .required(true && clap::ArgAction::SetTrue.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::impl_prelude::*;
                            let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                bool,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::SetTrue);
                    let arg = arg
                        .long("msgbox")
                        .value_name("enable message box")
                        .required(false)
                        .global(true);
                    let arg = arg;
                    arg
                });
            let __clap_app = <SubCommand as clap::Subcommand>::augment_subcommands(
                __clap_app,
            );
            let __clap_app = __clap_app;
            __clap_app
                .version("0.4.0")
                .about(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "\u{1b}[1m{0} version {1}\u{1b}[m\n{2}",
                                "yy-battery-notifier-rs",
                                "0.4.0",
                                "Check battery level & Notify you!",
                            ),
                        )
                    }),
                )
                .author("yy-tromb")
                .long_about(None)
                .disable_help_subcommand(true)
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("AppArgs")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 3usize] = [
                                clap::Id::from("toml_settings_path"),
                                clap::Id::from("default_settings"),
                                clap::Id::from("msgbox"),
                            ];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("toml_settings_path")
                        .value_name("TOML_SETTINGS_PATH")
                        .required(false && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::impl_prelude::*;
                            let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                String,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .short('s')
                        .long("settings")
                        .value_name("path to settings.toml")
                        .default_value({
                            static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                            let s = DEFAULT_VALUE
                                .get_or_init(|| {
                                    let val: String = String::from(".\\settings.toml");
                                    ::std::string::ToString::to_string(&val)
                                });
                            let s: &'static str = &*s;
                            s
                        });
                    let arg = arg.required(false);
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("default_settings")
                        .value_name("DEFAULT_SETTINGS")
                        .required(true && clap::ArgAction::SetTrue.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::impl_prelude::*;
                            let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                bool,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::SetTrue);
                    let arg = arg
                        .short('d')
                        .long("default_settings")
                        .value_name("use default_settings.toml")
                        .required(false);
                    let arg = arg.required(false);
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("msgbox")
                        .value_name("MSGBOX")
                        .required(true && clap::ArgAction::SetTrue.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::impl_prelude::*;
                            let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                bool,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::SetTrue);
                    let arg = arg
                        .long("msgbox")
                        .value_name("enable message box")
                        .required(false)
                        .global(true);
                    let arg = arg.required(false);
                    arg
                });
            let __clap_app = <SubCommand as clap::Subcommand>::augment_subcommands(
                __clap_app,
            );
            let __clap_app = __clap_app
                .subcommand_required(false)
                .arg_required_else_help(false);
            __clap_app
                .version("0.4.0")
                .about(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "\u{1b}[1m{0} version {1}\u{1b}[m\n{2}",
                                "yy-battery-notifier-rs",
                                "0.4.0",
                                "Check battery level & Notify you!",
                            ),
                        )
                    }),
                )
                .author("yy-tromb")
                .long_about(None)
                .disable_help_subcommand(true)
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for AppArgs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "AppArgs",
            "toml_settings_path",
            &self.toml_settings_path,
            "default_settings",
            &self.default_settings,
            "msgbox",
            &self.msgbox,
            "subcommands",
            &&self.subcommands,
        )
    }
}
enum SubCommand {
    Aumid { #[command(subcommand)] subcommands: AumidSubCommand },
    Startup { #[command(subcommand)] subcommands: StartupSubCommand },
}
#[automatically_derived]
impl ::core::fmt::Debug for SubCommand {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            SubCommand::Aumid { subcommands: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Aumid",
                    "subcommands",
                    &__self_0,
                )
            }
            SubCommand::Startup { subcommands: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Startup",
                    "subcommands",
                    &__self_0,
                )
            }
        }
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::FromArgMatches for SubCommand {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        if let Some((__clap_name, mut __clap_arg_sub_matches)) = __clap_arg_matches
            .remove_subcommand()
        {
            let __clap_arg_matches = &mut __clap_arg_sub_matches;
            if __clap_name == "aumid" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::Aumid {
                    subcommands: {
                        <AumidSubCommand as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?
                    },
                });
            }
            if __clap_name == "startup" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::Startup {
                    subcommands: {
                        <StartupSubCommand as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?
                    },
                });
            }
            ::std::result::Result::Err(
                clap::Error::raw(
                    clap::error::ErrorKind::InvalidSubcommand,
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "the subcommand \'{0}\' wasn\'t recognized",
                                __clap_name,
                            ),
                        )
                    }),
                ),
            )
        } else {
            ::std::result::Result::Err(
                clap::Error::raw(
                    clap::error::ErrorKind::MissingSubcommand,
                    "a subcommand is required but one was not provided",
                ),
            )
        }
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut<'b>(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
            match self {
                Self::Aumid { subcommands } if "aumid" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                        .remove_subcommand()
                        .unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {
                        {
                            <AumidSubCommand as clap::FromArgMatches>::update_from_arg_matches_mut(
                                subcommands,
                                __clap_arg_matches,
                            )?;
                        }
                    }
                }
                Self::Startup { subcommands } if "startup" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                        .remove_subcommand()
                        .unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {
                        {
                            <StartupSubCommand as clap::FromArgMatches>::update_from_arg_matches_mut(
                                subcommands,
                                __clap_arg_matches,
                            )?;
                        }
                    }
                }
                s => {
                    *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                        __clap_arg_matches,
                    )?;
                }
            }
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::Subcommand for SubCommand {
    fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("aumid");
                {
                    let __clap_subcommand = __clap_subcommand
                        .group(
                            clap::ArgGroup::new("Aumid")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 0usize] = [];
                                    members
                                }),
                        );
                    let __clap_subcommand = <AumidSubCommand as clap::Subcommand>::augment_subcommands(
                        __clap_subcommand,
                    );
                    let __clap_subcommand = __clap_subcommand
                        .subcommand_required(true)
                        .arg_required_else_help(true);
                    __clap_subcommand
                }
            });
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("startup");
                {
                    let __clap_subcommand = __clap_subcommand
                        .group(
                            clap::ArgGroup::new("Startup")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 0usize] = [];
                                    members
                                }),
                        );
                    let __clap_subcommand = <StartupSubCommand as clap::Subcommand>::augment_subcommands(
                        __clap_subcommand,
                    );
                    let __clap_subcommand = __clap_subcommand
                        .subcommand_required(true)
                        .arg_required_else_help(true);
                    __clap_subcommand
                }
            });
        __clap_app
    }
    fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("aumid");
                {
                    let __clap_subcommand = __clap_subcommand
                        .group(
                            clap::ArgGroup::new("Aumid")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 0usize] = [];
                                    members
                                }),
                        );
                    let __clap_subcommand = <AumidSubCommand as clap::Subcommand>::augment_subcommands(
                        __clap_subcommand,
                    );
                    let __clap_subcommand = __clap_subcommand
                        .subcommand_required(true)
                        .arg_required_else_help(true)
                        .subcommand_required(false)
                        .arg_required_else_help(false);
                    __clap_subcommand
                }
            });
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("startup");
                {
                    let __clap_subcommand = __clap_subcommand
                        .group(
                            clap::ArgGroup::new("Startup")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 0usize] = [];
                                    members
                                }),
                        );
                    let __clap_subcommand = <StartupSubCommand as clap::Subcommand>::augment_subcommands(
                        __clap_subcommand,
                    );
                    let __clap_subcommand = __clap_subcommand
                        .subcommand_required(true)
                        .arg_required_else_help(true)
                        .subcommand_required(false)
                        .arg_required_else_help(false);
                    __clap_subcommand
                }
            });
        __clap_app
    }
    fn has_subcommand(__clap_name: &str) -> bool {
        if "aumid" == __clap_name {
            return true;
        }
        if "startup" == __clap_name {
            return true;
        }
        false
    }
}
enum AumidSubCommand {
    Register,
    Delete,
}
#[automatically_derived]
impl ::core::fmt::Debug for AumidSubCommand {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                AumidSubCommand::Register => "Register",
                AumidSubCommand::Delete => "Delete",
            },
        )
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::FromArgMatches for AumidSubCommand {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        if let Some((__clap_name, mut __clap_arg_sub_matches)) = __clap_arg_matches
            .remove_subcommand()
        {
            let __clap_arg_matches = &mut __clap_arg_sub_matches;
            if __clap_name == "register" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::Register);
            }
            if __clap_name == "delete" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::Delete);
            }
            ::std::result::Result::Err(
                clap::Error::raw(
                    clap::error::ErrorKind::InvalidSubcommand,
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "the subcommand \'{0}\' wasn\'t recognized",
                                __clap_name,
                            ),
                        )
                    }),
                ),
            )
        } else {
            ::std::result::Result::Err(
                clap::Error::raw(
                    clap::error::ErrorKind::MissingSubcommand,
                    "a subcommand is required but one was not provided",
                ),
            )
        }
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut<'b>(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
            match self {
                Self::Register if "register" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                        .remove_subcommand()
                        .unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {}
                }
                Self::Delete if "delete" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                        .remove_subcommand()
                        .unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {}
                }
                s => {
                    *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                        __clap_arg_matches,
                    )?;
                }
            }
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::Subcommand for AumidSubCommand {
    fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("register");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = __clap_subcommand;
                __clap_subcommand
            });
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("delete");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = __clap_subcommand;
                __clap_subcommand
            });
        __clap_app
    }
    fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("register");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = __clap_subcommand;
                __clap_subcommand
            });
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("delete");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = __clap_subcommand;
                __clap_subcommand
            });
        __clap_app
    }
    fn has_subcommand(__clap_name: &str) -> bool {
        if "register" == __clap_name {
            return true;
        }
        if "delete" == __clap_name {
            return true;
        }
        false
    }
}
enum StartupSubCommand {
    Register {
        #[arg(short = 's', long = "settings", value_name = "path to settings.toml")]
        toml_settings_path: Option<String>,
        #[arg(
            short = 'i',
            long = "input",
            value_name = "Whether you input path of settings.toml in terminal or not."
        )]
        input_mode: bool,
        #[arg(
            short = 'd',
            long = "default_settings",
            value_name = "use default_settings.toml",
            required = false
        )]
        default_settings: bool,
    },
    Delete,
}
#[automatically_derived]
impl ::core::fmt::Debug for StartupSubCommand {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            StartupSubCommand::Register {
                toml_settings_path: __self_0,
                input_mode: __self_1,
                default_settings: __self_2,
            } => {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Register",
                    "toml_settings_path",
                    __self_0,
                    "input_mode",
                    __self_1,
                    "default_settings",
                    &__self_2,
                )
            }
            StartupSubCommand::Delete => ::core::fmt::Formatter::write_str(f, "Delete"),
        }
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::FromArgMatches for StartupSubCommand {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        if let Some((__clap_name, mut __clap_arg_sub_matches)) = __clap_arg_matches
            .remove_subcommand()
        {
            let __clap_arg_matches = &mut __clap_arg_sub_matches;
            if __clap_name == "register" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::Register {
                    toml_settings_path: __clap_arg_matches
                        .remove_one::<String>("toml_settings_path"),
                    input_mode: __clap_arg_matches
                        .remove_one::<bool>("input_mode")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "the following required argument was not provided: input_mode",
                        ))?,
                    default_settings: __clap_arg_matches
                        .remove_one::<bool>("default_settings")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "the following required argument was not provided: default_settings",
                        ))?,
                });
            }
            if __clap_name == "delete" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::Delete);
            }
            ::std::result::Result::Err(
                clap::Error::raw(
                    clap::error::ErrorKind::InvalidSubcommand,
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "the subcommand \'{0}\' wasn\'t recognized",
                                __clap_name,
                            ),
                        )
                    }),
                ),
            )
        } else {
            ::std::result::Result::Err(
                clap::Error::raw(
                    clap::error::ErrorKind::MissingSubcommand,
                    "a subcommand is required but one was not provided",
                ),
            )
        }
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut<'b>(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
            match self {
                Self::Register {
                    toml_settings_path,
                    input_mode,
                    default_settings,
                } if "register" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                        .remove_subcommand()
                        .unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {
                        if __clap_arg_matches.contains_id("toml_settings_path") {
                            *toml_settings_path = __clap_arg_matches
                                .remove_one::<String>("toml_settings_path");
                        }
                        if __clap_arg_matches.contains_id("input_mode") {
                            *input_mode = __clap_arg_matches
                                .remove_one::<bool>("input_mode")
                                .ok_or_else(|| clap::Error::raw(
                                    clap::error::ErrorKind::MissingRequiredArgument,
                                    "the following required argument was not provided: input_mode",
                                ))?;
                        }
                        if __clap_arg_matches.contains_id("default_settings") {
                            *default_settings = __clap_arg_matches
                                .remove_one::<bool>("default_settings")
                                .ok_or_else(|| clap::Error::raw(
                                    clap::error::ErrorKind::MissingRequiredArgument,
                                    "the following required argument was not provided: default_settings",
                                ))?;
                        }
                    }
                }
                Self::Delete if "delete" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                        .remove_subcommand()
                        .unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {}
                }
                s => {
                    *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                        __clap_arg_matches,
                    )?;
                }
            }
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::Subcommand for StartupSubCommand {
    fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("register");
                {
                    let __clap_subcommand = __clap_subcommand
                        .group(
                            clap::ArgGroup::new("Register")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 3usize] = [
                                        clap::Id::from("toml_settings_path"),
                                        clap::Id::from("input_mode"),
                                        clap::Id::from("default_settings"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_subcommand = __clap_subcommand
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("toml_settings_path")
                                .value_name("TOML_SETTINGS_PATH")
                                .value_parser({
                                    use ::clap_builder::builder::impl_prelude::*;
                                    let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .short('s')
                                .long("settings")
                                .value_name("path to settings.toml");
                            let arg = arg;
                            arg
                        });
                    let __clap_subcommand = __clap_subcommand
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("input_mode")
                                .value_name("INPUT_MODE")
                                .required(true && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::impl_prelude::*;
                                    let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .short('i')
                                .long("input")
                                .value_name(
                                    "Whether you input path of settings.toml in terminal or not.",
                                );
                            let arg = arg;
                            arg
                        });
                    let __clap_subcommand = __clap_subcommand
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("default_settings")
                                .value_name("DEFAULT_SETTINGS")
                                .required(true && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::impl_prelude::*;
                                    let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .short('d')
                                .long("default_settings")
                                .value_name("use default_settings.toml")
                                .required(false);
                            let arg = arg;
                            arg
                        });
                    __clap_subcommand
                }
            });
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("delete");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = __clap_subcommand;
                __clap_subcommand
            });
        __clap_app
    }
    fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("register");
                {
                    let __clap_subcommand = __clap_subcommand
                        .group(
                            clap::ArgGroup::new("Register")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 3usize] = [
                                        clap::Id::from("toml_settings_path"),
                                        clap::Id::from("input_mode"),
                                        clap::Id::from("default_settings"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_subcommand = __clap_subcommand
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("toml_settings_path")
                                .value_name("TOML_SETTINGS_PATH")
                                .value_parser({
                                    use ::clap_builder::builder::impl_prelude::*;
                                    let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .short('s')
                                .long("settings")
                                .value_name("path to settings.toml");
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_subcommand = __clap_subcommand
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("input_mode")
                                .value_name("INPUT_MODE")
                                .required(true && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::impl_prelude::*;
                                    let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .short('i')
                                .long("input")
                                .value_name(
                                    "Whether you input path of settings.toml in terminal or not.",
                                );
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_subcommand = __clap_subcommand
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("default_settings")
                                .value_name("DEFAULT_SETTINGS")
                                .required(true && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::impl_prelude::*;
                                    let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .short('d')
                                .long("default_settings")
                                .value_name("use default_settings.toml")
                                .required(false);
                            let arg = arg.required(false);
                            arg
                        });
                    __clap_subcommand
                }
            });
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("delete");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = __clap_subcommand;
                __clap_subcommand
            });
        __clap_app
    }
    fn has_subcommand(__clap_name: &str) -> bool {
        if "register" == __clap_name {
            return true;
        }
        if "delete" == __clap_name {
            return true;
        }
        false
    }
}
fn main() -> anyhow::Result<()> {
    let app_args = AppArgs::parse();
    if let Some(subcommand) = app_args.subcommands {
        match subcommand {
            SubCommand::Aumid { subcommands } => {
                match subcommands {
                    AumidSubCommand::Register => {
                        if app_args.msgbox {
                            crate::aumid::register_and_check_aumid(CURRENT_USER)
                                .with_context(|| {
                                    let path = "src\\main.rs";
                                    let line = 156usize;
                                    let col = 98usize;
                                    let expr = " 156>    crate::aumid::register_and_check_aumid(CURRENT_USER)?\n    |";
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                        )
                                    })
                                })
                                .or_else(|e| {
                                    Err(
                                        crate::common::error_msgbox(&e)
                                            .inspect_err(|_| {
                                                ::std::io::_eprint(format_args!("{0:?}\n", e));
                                            })
                                            .err()
                                            .unwrap_or(e),
                                    )
                                })
                        } else {
                            crate::aumid::register_and_check_aumid(CURRENT_USER)
                                .with_context(|| {
                                    let path = "src\\main.rs";
                                    let line = 156usize;
                                    let col = 98usize;
                                    let expr = " 156>    crate::aumid::register_and_check_aumid(CURRENT_USER)?\n    |";
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                        )
                                    })
                                })
                        }?
                    }
                    AumidSubCommand::Delete => {
                        if app_args.msgbox {
                            crate::aumid::delete_and_check_aumid(CURRENT_USER)
                                .with_context(|| {
                                    let path = "src\\main.rs";
                                    let line = 157usize;
                                    let col = 94usize;
                                    let expr = " 157>    crate::aumid::delete_and_check_aumid(CURRENT_USER)?\n    |";
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                        )
                                    })
                                })
                                .or_else(|e| {
                                    Err(
                                        crate::common::error_msgbox(&e)
                                            .inspect_err(|_| {
                                                ::std::io::_eprint(format_args!("{0:?}\n", e));
                                            })
                                            .err()
                                            .unwrap_or(e),
                                    )
                                })
                        } else {
                            crate::aumid::delete_and_check_aumid(CURRENT_USER)
                                .with_context(|| {
                                    let path = "src\\main.rs";
                                    let line = 157usize;
                                    let col = 94usize;
                                    let expr = " 157>    crate::aumid::delete_and_check_aumid(CURRENT_USER)?\n    |";
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                        )
                                    })
                                })
                        }?
                    }
                }
            }
            SubCommand::Startup { subcommands } => {
                match subcommands {
                    StartupSubCommand::Register {
                        toml_settings_path,
                        input_mode,
                        default_settings,
                    } => {
                        if app_args.msgbox {
                            crate::startup::register_cli(
                                    toml_settings_path,
                                    input_mode,
                                    default_settings,
                                )
                                .with_context(|| {
                                    let path = "src\\main.rs";
                                    let line = 165usize;
                                    let col = 99usize;
                                    let expr = " 165>    crate::startup::register_cli(toml_settings_path, input_mode, default_settings)?\n    |";
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                        )
                                    })
                                })
                                .or_else(|e| {
                                    Err(
                                        crate::common::error_msgbox(&e)
                                            .inspect_err(|_| {
                                                ::std::io::_eprint(format_args!("{0:?}\n", e));
                                            })
                                            .err()
                                            .unwrap_or(e),
                                    )
                                })
                        } else {
                            crate::startup::register_cli(
                                    toml_settings_path,
                                    input_mode,
                                    default_settings,
                                )
                                .with_context(|| {
                                    let path = "src\\main.rs";
                                    let line = 165usize;
                                    let col = 99usize;
                                    let expr = " 165>    crate::startup::register_cli(toml_settings_path, input_mode, default_settings)?\n    |";
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                        )
                                    })
                                })
                        }?
                    }
                    StartupSubCommand::Delete => {
                        if app_args.msgbox {
                            crate::startup::delete_and_check_startup()
                                .with_context(|| {
                                    let path = "src\\main.rs";
                                    let line = 167usize;
                                    let col = 88usize;
                                    let expr = " 167>    crate::startup::delete_and_check_startup()?\n    |";
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                        )
                                    })
                                })
                                .or_else(|e| {
                                    Err(
                                        crate::common::error_msgbox(&e)
                                            .inspect_err(|_| {
                                                ::std::io::_eprint(format_args!("{0:?}\n", e));
                                            })
                                            .err()
                                            .unwrap_or(e),
                                    )
                                })
                        } else {
                            crate::startup::delete_and_check_startup()
                                .with_context(|| {
                                    let path = "src\\main.rs";
                                    let line = 167usize;
                                    let col = 88usize;
                                    let expr = " 167>    crate::startup::delete_and_check_startup()?\n    |";
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                        )
                                    })
                                })
                        }?
                    }
                }
            }
        };
        return Ok(());
    }
    let toml_settings_path = if app_args.default_settings {
        if app_args.msgbox {
            std::env::current_exe()
                .with_context(|| "Failed to get current execution file path.")
                .with_context(|| {
                    let path = "src\\main.rs";
                    let line = 174usize;
                    let col = 75usize;
                    let expr = " 173>    std::env::current_exe()\n 174|            .with_context(|| \"Fail..ath.\")?\n    |";
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                        )
                    })
                })
                .or_else(|e| {
                    Err(
                        crate::common::error_msgbox(&e)
                            .inspect_err(|_| {
                                ::std::io::_eprint(format_args!("{0:?}\n", e));
                            })
                            .err()
                            .unwrap_or(e),
                    )
                })
        } else {
            std::env::current_exe()
                .with_context(|| "Failed to get current execution file path.")
                .with_context(|| {
                    let path = "src\\main.rs";
                    let line = 174usize;
                    let col = 75usize;
                    let expr = " 173>    std::env::current_exe()\n 174|            .with_context(|| \"Fail..ath.\")?\n    |";
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                        )
                    })
                })
        }?
            .with_file_name("default_settings.toml")
    } else {
        std::path::PathBuf::from(&app_args.toml_settings_path)
    };
    match &toml_settings_path {
        tmp => {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3} = {4:#?}\n",
                        "src\\main.rs",
                        179u32,
                        5u32,
                        "& toml_settings_path",
                        &&tmp as &dyn ::std::fmt::Debug,
                    ),
                );
            };
            tmp
        }
    };
    let toml_settings = if app_args.msgbox {
        std::fs::read_to_string(&toml_settings_path)
            .with_context(|| {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "Failed to read file {0:?}.",
                                &toml_settings_path,
                            ),
                        )
                    })
                    .red()
            })
            .with_context(|| {
                let path = "src\\main.rs";
                let line = 181usize;
                let col = 90usize;
                let expr = " 180>    std::fs::read_to_string(&toml_settings_path)\n 181|        .with_context(|| format!(\"Failed to read file {:?}.\", &toml_settings_path).red())?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })
            .or_else(|e| {
                Err(
                    crate::common::error_msgbox(&e)
                        .inspect_err(|_| {
                            ::std::io::_eprint(format_args!("{0:?}\n", e));
                        })
                        .err()
                        .unwrap_or(e),
                )
            })
    } else {
        std::fs::read_to_string(&toml_settings_path)
            .with_context(|| {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "Failed to read file {0:?}.",
                                &toml_settings_path,
                            ),
                        )
                    })
                    .red()
            })
            .with_context(|| {
                let path = "src\\main.rs";
                let line = 181usize;
                let col = 90usize;
                let expr = " 180>    std::fs::read_to_string(&toml_settings_path)\n 181|        .with_context(|| format!(\"Failed to read file {:?}.\", &toml_settings_path).red())?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })
    }?;
    let toml_settings: crate::settings::TOMLSettings = if app_args.msgbox {
        toml::from_str(&toml_settings)
            .with_context(|| {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "Failed to interpret \'{0}\' as TOML.",
                                &app_args.toml_settings_path,
                            ),
                        )
                    })
                    .red()
            })
            .with_context(|| {
                let path = "src\\main.rs";
                let line = 189usize;
                let col = 11usize;
                let expr = " 182>    toml::from_str(&toml_settings)\n...\n 186|                &app_args.toml_settings_path\n 187|            )\n 188|            .red()\n 189|        })?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })
            .or_else(|e| {
                Err(
                    crate::common::error_msgbox(&e)
                        .inspect_err(|_| {
                            ::std::io::_eprint(format_args!("{0:?}\n", e));
                        })
                        .err()
                        .unwrap_or(e),
                )
            })
    } else {
        toml::from_str(&toml_settings)
            .with_context(|| {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "Failed to interpret \'{0}\' as TOML.",
                                &app_args.toml_settings_path,
                            ),
                        )
                    })
                    .red()
            })
            .with_context(|| {
                let path = "src\\main.rs";
                let line = 189usize;
                let col = 11usize;
                let expr = " 182>    toml::from_str(&toml_settings)\n...\n 186|                &app_args.toml_settings_path\n 187|            )\n 188|            .red()\n 189|        })?\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })
    }?;
    if app_args.msgbox {
        crate::runner::Runner::new(
                if app_args.msgbox {
                    toml_settings
                        .try_into()
                        .with_context(|| {
                            let path = "src\\main.rs";
                            let line = 190usize;
                            let col = 56usize;
                            let expr = " 190>    toml_settings.try_into()?\n    |";
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                )
                            })
                        })
                        .or_else(|e| {
                            Err(
                                crate::common::error_msgbox(&e)
                                    .inspect_err(|_| {
                                        ::std::io::_eprint(format_args!("{0:?}\n", e));
                                    })
                                    .err()
                                    .unwrap_or(e),
                            )
                        })
                } else {
                    toml_settings
                        .try_into()
                        .with_context(|| {
                            let path = "src\\main.rs";
                            let line = 190usize;
                            let col = 56usize;
                            let expr = " 190>    toml_settings.try_into()?\n    |";
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                )
                            })
                        })
                }?,
            )
            .run()
            .with_context(|| {
                let path = "src\\main.rs";
                let line = 190usize;
                let col = 5usize;
                let expr = " 190>    crate::runner::Runner::new(toml_settings.try_into()?).run()\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })
            .or_else(|e| {
                Err(
                    crate::common::error_msgbox(&e)
                        .inspect_err(|_| {
                            ::std::io::_eprint(format_args!("{0:?}\n", e));
                        })
                        .err()
                        .unwrap_or(e),
                )
            })
    } else {
        crate::runner::Runner::new(
                if app_args.msgbox {
                    toml_settings
                        .try_into()
                        .with_context(|| {
                            let path = "src\\main.rs";
                            let line = 190usize;
                            let col = 56usize;
                            let expr = " 190>    toml_settings.try_into()?\n    |";
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                )
                            })
                        })
                        .or_else(|e| {
                            Err(
                                crate::common::error_msgbox(&e)
                                    .inspect_err(|_| {
                                        ::std::io::_eprint(format_args!("{0:?}\n", e));
                                    })
                                    .err()
                                    .unwrap_or(e),
                            )
                        })
                } else {
                    toml_settings
                        .try_into()
                        .with_context(|| {
                            let path = "src\\main.rs";
                            let line = 190usize;
                            let col = 56usize;
                            let expr = " 190>    toml_settings.try_into()?\n    |";
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                                )
                            })
                        })
                }?,
            )
            .run()
            .with_context(|| {
                let path = "src\\main.rs";
                let line = 190usize;
                let col = 5usize;
                let expr = " 190>    crate::runner::Runner::new(toml_settings.try_into()?).run()\n    |";
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("[{0}:{1}:{2}]\n{3}", path, line, col, expr),
                    )
                })
            })
    }
}
