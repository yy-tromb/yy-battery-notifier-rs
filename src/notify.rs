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
            notify_tauri_winrt_toast(battery_report, title, message)
        }
    }
}

fn notify_tauri_winrt_toast(
    battery_report: &crate::battery::BatteryReport,
    title: &str,
    message: &str,
) -> anyhow::Result<Option<NotificationAction>> {
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
    let mut notification_action: Option<NotificationAction> = None;
    Toast::new("yy-tromb.yy-battery-notifier-rs")
        .title(title)
        .text1(message)
        .progress(&progress)
        //.add_button("temporary1", "temporary action 1")
        //.add_button("temporary2", "temporary action 2")
        /*.on_activated(|action| {
            match action.as_deref() {
                Some("temporary action 1") => {
                    println!("Temporary action 1 executed.");
                    notification_action = Some(NotificationAction::Temporary1);
                }
                Some("temporary action 2") => {
                    println!("Temporary action 2 executed.");
                }
                Some(action) => {
                    println!("Unknown action: {}", action);
                }
                None => {
                    println!("Toast activated without action.");
                }
            }
            Ok(())
        })*/
        .duration(Duration::Short)
        .show()
        .map_err(|error| match error {
            tauri_winrt_notification::Error::Os(e) => anyhow::Error::from(e),
            tauri_winrt_notification::Error::Io(e) => anyhow::Error::from(e),
        })?;
    Ok(notification_action)
}
