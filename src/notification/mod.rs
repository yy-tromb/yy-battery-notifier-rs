mod tauri_winrt;

pub enum NotificationMethod {
    TauriWinrtToast,
    // Future methods can be added here
}

pub enum NotificationAction {
    Temporary1,
    Temporary2,
}

#[inline]
pub fn notify(
    battery_report: &crate::battery::BatteryReport,
    title: &str,
    message: &str,
    method: NotificationMethod,
) -> anyhow::Result<Option<NotificationAction>> {
    match method {
        NotificationMethod::TauriWinrtToast => {
            tauri_winrt::notify_tauri_winrt_toast(battery_report, title, message)
        }
    }
}