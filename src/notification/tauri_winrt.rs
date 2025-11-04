use super::NotificationAction;
use std::sync::{Arc, RwLock};

pub(super) fn notify_tauri_winrt_toast(
    battery_report: &crate::battery::BatteryReport,
    title: &str,
    message: &str,
    notification_action: Arc<RwLock<Option<NotificationAction>>>,
) -> anyhow::Result<()> {
    use tauri_winrt_notification::{Duration, Progress, Toast};
    let progress = match battery_report.remaining_seconds {
        Some(remaining_seconds) => Progress {
            tag: "tag".to_string(),
            title: "Now Battery Level:".to_string(),
            status: format!(
                "{}:{}:{} remaining",
                remaining_seconds / 3600,
                (remaining_seconds % 3600) / 60,
                remaining_seconds % 60
            ),
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
        .add_button("silent for 5 mins", "silent 5 mins")
        .add_button("silent for 10 mins", "silent 10 mins")
        .on_activated(move |action| {
            match action.as_deref() {
                Some("silent 5 mins") => {
                    println!("Toast activated with action: silent 5 mins");
                    if let Ok(mut action_guard) = notification_action.write() {
                        *action_guard = Some(NotificationAction::Silent5Mins);
                    }
                }
                Some("silent 10 mins") => {
                    println!("Toast activated with action: silent 10 mins");
                    if let Ok(mut action_guard) = notification_action.write() {
                        *action_guard = Some(NotificationAction::Silent10Mins);
                    }
                }
                Some(unknown_action) => {
                    println!("Toast activated with unknown action: {}", unknown_action);
                }
                None => {
                    println!("Toast activated without action.");
                }
            }
            Ok(())
        })
        .duration(Duration::Short)
        .show()
        .map_err(|error| match error {
            tauri_winrt_notification::Error::Os(e) => anyhow::Error::from(e),
            tauri_winrt_notification::Error::Io(e) => anyhow::Error::from(e),
        })?;
    Ok(())
}
