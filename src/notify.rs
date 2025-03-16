#[inline]
pub fn notify(
    battery_report: &crate::battery::BatteryReport,
    title: &str,
    message: &str,
) -> anyhow::Result<()> {
    notify_winrt_toast(battery_report, title, message)
}

fn notify_winrt_toast(
    battery_report: &crate::battery::BatteryReport,
    title: &str,
    message: &str,
) -> anyhow::Result<()> {
    use tauri_winrt_notification::{Duration, Toast};
    Toast::new("yy-tromb.yy-battery-notifier-rs")
        .title(title)
        .text1(message)
        .show()
        .map_err(|tauri_error| match tauri_error {
            tauri_winrt_notification::Error::Os(e) => anyhow::anyhow!("Windows Error"), // ToDo
            tauri_winrt_notification::Error::Io(e) => anyhow::anyhow!("IO Error"),      // ToDo
        })
}
