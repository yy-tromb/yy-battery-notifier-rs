use super::NotificationAction;

const AUMID: &str = "yy-tromb.yy-battery-notifier-rs";

pub(super) fn notify_winrt_toast_reborn(
    battery_report: &crate::battery::BatteryReport,
    title: &str,
    message: &str,
) -> anyhow::Result<Option<NotificationAction>> {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use winrt_toast_reborn::{Action, Toast, ToastDuration, ToastManager};

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
        .action(Action::new("temporary1", "temporary action 1", ""))
        .action(Action::new("temporary2", "temporary action 2", ""));

    let action_take = Arc::new(AtomicBool::new(false));
    let action_clone = Arc::clone(&action_take);
    let dismiss_clone = Arc::clone(&action_take);

    fn handle_activated_action(action: Option<winrt_toast_reborn::ActivatedAction>) {
        match action {
            Some(action) => {
                let message = format!(
                    "Toast activated with action: {}",
                    action.arg
                );
                println!("{}", message);
            }
            None => {
                println!("Toast activated without action.");
            }
        }
    }

    toast_manager.on_activated(None, move |action| {
            handle_activated_action(action);
            action_clone.store(true, Ordering::SeqCst);
        })
    .show(&toast)?;
    Ok(None)
}
