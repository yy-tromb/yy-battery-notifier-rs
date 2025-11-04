use super::NotificationAction;
use std::{
    f32::consts::E,
    sync::{Arc, Mutex, RwLock},
};

const AUMID: &str = "yy-tromb.yy-battery-notifier-rs";

pub(super) fn notify_winrt_toast_reborn(
    battery_report: &crate::battery::BatteryReport,
    title: &str,
    message: &str,
    notification_action: Arc<Mutex<Option<NotificationAction>>>,
) -> anyhow::Result<()> {
    use std::sync::atomic::{AtomicBool, Ordering};
    use winrt_toast_reborn::content::input::InputType;
    use winrt_toast_reborn::{Action, Input, Selection, Toast, ToastDuration, ToastManager};

    let toast_manager = ToastManager::new(AUMID);

    /*let progress_value = battery_report.percentage as f32 / 100.0;
    let progress_status = match battery_report.remaining_seconds {
        Some(remaining_seconds) => format!(
            "{}:{}:{} remaining",
            remaining_seconds / 3600,
            (remaining_seconds % 3600) / 60,
            remaining_seconds % 60
        ),
        None => "N/A".to_string(),
    };*/

    let mut toast = Toast::new();
    toast.text1(title).text2(message);
    /*toast.progress(
        "tag",
        "Now Battery Level:",
        &progress_status,
        progress_value,
        &format!("{}%", battery_report.percentage),
    );*/
    toast
        .duration(ToastDuration::Short)
        .input(
            Input::new("silent_time", InputType::Text)
                .with_title("Select silent minites")
                .with_default_input("5"),
        )
        .action(Action::new("silent for 5 mins", "silent 5 mins", ""))
        .action(Action::new("silent for 10 mins", "silent 10 mins", ""))
        .action(
            Action::new(
                "silent for your specified mins",
                "silent specified mins",
                "",
            )
            .with_input_id("silent_time"),
        );
    //toast.action(Action::new("change mode", "change mode", ""));

    toast_manager
        .on_activated(None, move |action| {
            handle_activated_action(action, &notification_action);
        })
        .show(&toast)?;
    Ok(())
}

fn handle_activated_action(
    action: Option<winrt_toast_reborn::ActivatedAction>,
    notification_action: &Arc<Mutex<Option<NotificationAction>>>,
) {
    match action {
        Some(action) => {
            let message = format!("Toast activated with action: {}", action.arg);
            println!("{}", message);
            match action.arg.as_str() {
                "silent 5 mins" => {
                    if let Ok(mut guard) = notification_action.lock() {
                        println!("Setting silent time to 5 mins");
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
                            println!("Setting silent time to: {}", silent_time_int);
                            *guard = Some(NotificationAction::SilentSpecifiedMins(silent_time_int));
                        } else {
                            println!("Failed to parse silent time input: {}", silent_time);
                        }
                    } else {
                        println!("No input value found for silent time.");
                    }
                }
                _ => {
                    println!("Unknown action.");
                }
            }
        }
        None => {
            println!("Toast activated without action.");
        }
    }
}
