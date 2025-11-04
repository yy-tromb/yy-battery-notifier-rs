mod tauri_winrt;
mod winrt_toast_reborn;

use std::sync::{Arc, RwLock};

use crate::notification;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum NotificationMethod {
    TauriWinrtToast,
    WinrtToastReborn,
    // Future methods can be added here
}

#[derive(Debug, Clone)]
pub enum NotificationAction {
    Temporary1,
    Temporary2,
    Silent10Mins,
}

#[inline]
pub fn notify(
    battery_report: &crate::battery::BatteryReport,
    title: &str,
    message: &str,
    method: &NotificationMethod,
    notification_action: Arc<RwLock<Option<NotificationAction>>>,
) -> anyhow::Result<()> {
    match method {
        NotificationMethod::TauriWinrtToast => {
            tauri_winrt::notify_tauri_winrt_toast(battery_report, title, message,notification_action)
        },
        NotificationMethod::WinrtToastReborn => {
            winrt_toast_reborn::notify_winrt_toast_reborn(battery_report, title, message,notification_action)
        },
    }
}