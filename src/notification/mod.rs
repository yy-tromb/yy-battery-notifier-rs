mod tauri_winrt;
mod winrt_toast_reborn;

use std::sync::{Arc, Mutex};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum NotificationMethod {
    TauriWinrtToast,
    WinrtToastReborn,
    // Future methods can be added here
}

#[derive(Debug, Clone)]
pub enum NotificationAction {
    Silent5Mins,
    Silent10Mins,
    SilentSpecifiedMins(u64),
}

#[inline]
pub fn battery_notify(
    battery_report: &crate::battery::BatteryReport,
    title: &str,
    message: &str,
    method: &NotificationMethod,
    notification_action: Arc<Mutex<Option<NotificationAction>>>,
) -> anyhow::Result<()> {
    match method {
        NotificationMethod::TauriWinrtToast => {
            tauri_winrt::battery_notify_tauri_winrt_toast(battery_report, title, message,notification_action)
        },
        NotificationMethod::WinrtToastReborn => {
            winrt_toast_reborn::battery_notify_winrt_toast_reborn(battery_report, title, message,notification_action)
        },
    }
}