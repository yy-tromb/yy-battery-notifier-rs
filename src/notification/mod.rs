mod tauri_winrt_toast;
mod winrt_toast_reborn;

use std::sync::{Arc, Mutex};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub enum NotificationMethod {
    #[default]
    TauriWinrtToast,
    WinrtToastReborn,
    // Future methods can be added here
}

#[derive(Debug, Clone)]
pub enum NotificationAction {
    Silent5Mins,
    Silent10Mins,
    SilentSpecifiedMins(u64),
    RequireChangeMode,
    ChangeMode(String),
}

#[inline]
pub fn battery_notify(
    battery_report: &crate::battery::BatteryReport,
    title: &str,
    message: &str,
    method: &NotificationMethod,
    notification_action: Arc<Mutex<Option<NotificationAction>>>,
    mode_names: &[&String],
    mode: &str,
) -> anyhow::Result<()> {
    match method {
        NotificationMethod::TauriWinrtToast => tauri_winrt_toast::battery_notify_tauri_winrt_toast(
            battery_report,
            title,
            message,
            notification_action,
        ),
        NotificationMethod::WinrtToastReborn => {
            winrt_toast_reborn::battery_notify_winrt_toast_reborn(
                battery_report,
                title,
                message,
                notification_action,
                mode_names,
                mode,
            )
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
            return (battery_report.percentage > notification_setting.percentage_int)
                && (battery_report.power_supply == notification_setting.power_supply);
        }
        crate::settings::PercentageSymbol::Under => {
            return (battery_report.percentage < notification_setting.percentage_int)
                && (battery_report.power_supply == notification_setting.power_supply);
        }
    };
}
