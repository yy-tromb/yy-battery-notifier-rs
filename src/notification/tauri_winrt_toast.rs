use super::NotificationAction;
use std::sync::{Arc, Mutex};
use tauri_winrt_notification::{Duration, Progress, Toast};

pub(super) fn battery_notify_tauri_winrt_toast(
    battery_report: &crate::battery::BatteryReport,
    title: &str,
    message: &str,
    notification_action: Arc<Mutex<Option<NotificationAction>>>,
) -> anyhow::Result<()> {
    let progress = match battery_report.remaining_seconds {
        Some(remaining_seconds) => Progress {
            tag: "battery_progress".to_string(),
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
            tag: "battery_progress".to_string(),
            title: "Now Battery Level:".to_string(),
            status: "Unknown time remaining".to_string(),
            value: battery_report.percentage as f32 / 100.0,
            value_string: format!("{}%", battery_report.percentage),
        },
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
    toast.show().map_err(|error| match error {
        tauri_winrt_notification::Error::Os(e) => anyhow::Error::from(e),
        tauri_winrt_notification::Error::Io(e) => anyhow::Error::from(e),
    })?;
    Ok(())
}

fn handle_battery_notify_activated_action(
    action: Option<String>,
    notification_action: &Arc<Mutex<Option<NotificationAction>>>,
) {
    match action.as_deref() {
        Some("silent 5 mins") => {
            println!("Toast activated with action: silent 5 mins");
            if let Ok(mut action_guard) = notification_action.lock() {
                *action_guard = Some(NotificationAction::Silent5Mins);
            }
        }
        Some("silent 10 mins") => {
            println!("Toast activated with action: silent 10 mins");
            if let Ok(mut action_guard) = notification_action.lock() {
                *action_guard = Some(NotificationAction::Silent10Mins);
            }
        }
        Some("require change mode") => {
            println!("Toast activated with action: require change mode");
            if let Ok(mut action_guard) = notification_action.lock() {
                *action_guard = Some(NotificationAction::RequireChangeMode);
            }
        }
        Some(unknown_action) => {
            println!("Toast activated with unknown action: {}", unknown_action);
        }
        None => {
            println!("Toast activated without action.");
        }
    }
}

pub(super) fn mode_change_notify_tauri_winrt_toast(
    notification_action: Arc<Mutex<Option<NotificationAction>>>,
    mode_names: &[&String],
) -> anyhow::Result<()> {
    let mut toast = Toast::new(crate::aumid::AUMID)
        .title("Notify Mode Change")
        .duration(Duration::Long)
        .add_button("&lt;no mode&gt;", "mode_no_mode");
    for mode_name in mode_names {
        toast = toast.add_button(mode_name, format!("mode:{}", mode_name).as_str());
    }

    toast = toast.on_activated(move |action| {
        handle_mode_change_notify_activated_action(action, &notification_action);
        Ok(())
    });

    toast.show().map_err(|error| match error {
        tauri_winrt_notification::Error::Os(e) => anyhow::Error::from(e),
        tauri_winrt_notification::Error::Io(e) => anyhow::Error::from(e),
    })?;
    Ok(())
}

fn handle_mode_change_notify_activated_action(
    action: Option<String>,
    notification_action: &Arc<Mutex<Option<NotificationAction>>>,
) {
    match action.as_deref() {
        Some("mode_no_mode") => {
            println!("change to no mode");
            let mut guard = match notification_action.lock() {
                Ok(guard) => guard,
                Err(e) => e.into_inner(),
            };
            *guard = Some(NotificationAction::ChangeMode(String::default()));
        }
        Some(action) => {
            println!(r#"change mode to id="{}""#, action);
            let mut guard = match notification_action.lock() {
                Ok(guard) => guard,
                Err(e) => e.into_inner(),
            };
            *guard = Some(NotificationAction::ChangeMode(
                action.get(5..).unwrap_or_default().to_string(),
            ));
        }
        None => {
            println!("Toast activated without action.");
        }
    }
}
