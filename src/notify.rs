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
    use tauri_winrt_notification::{Duration, Progress, Toast};
    let progress = match battery_report.remaining_seconds {
        Some(remaining_seconds) => Progress {
            tag: "tag".to_string(),
            title: "Now Battery Level:".to_string(),
            status: format!("{}:{}:{} remaining", remaining_seconds / 3600, (remaining_seconds % 3600) / 60, remaining_seconds % 60),
            value: battery_report.percentage as f32 / 100.0,
            value_string: format!("{}%", battery_report.percentage),
        },
        None => Progress {
            tag: "tag".to_string(),
            title: "Now Battery Level:".to_string(),
            status: "N/A".to_string(),
            value: battery_report.percentage as f32 / 100.0,
            value_string: format!("{}%", battery_report.percentage),
        },
    };
    Toast::new("yy-tromb.yy-battery-notifier-rs")
        .title(title)
        .text1(message)
        .progress(&progress)
        .add_button("temporary1", "temporary action 1")
        .add_button("temporary2", "temporary action 2")
        .on_activated(|action| {
            println!("Toast activated: {}", action.unwrap_or("no action".to_string()));
            tauri_winrt_notification::Result::Ok(())
        })
        .duration(Duration::Short)
        .show()
        .map_err(|error| match error {
            tauri_winrt_notification::Error::Os(e) => anyhow::Error::from(e),
            tauri_winrt_notification::Error::Io(e) => anyhow::Error::from(e),
        })
}
