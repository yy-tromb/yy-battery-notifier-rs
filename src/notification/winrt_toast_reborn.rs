use anyhow::Context as _;
use colored::Colorize;
use hooq::hooq;
use winrt_toast_reborn::content::input::InputType;
use winrt_toast_reborn::{Action, Input, Selection, Toast, ToastDuration, ToastManager};

use std::sync::{Arc, Mutex};

use super::{NotificationAction, NotificationInputType};

#[hooq(anyhow)]
pub(super) fn battery_notify_winrt_toast_reborn(
    battery_report: &crate::battery::BatteryReport,
    title: &str,
    message: &str,
    notification_action: Arc<Mutex<Option<NotificationAction>>>,
    input_type: &NotificationInputType,
    mode_names: &[String],
) -> anyhow::Result<()> {
    let toast_manager = ToastManager::new(crate::aumid::AUMID);

    //let progress_value = battery_report.percentage as f32 / 100.0;
    let battery_remaining_status = match battery_report.remaining_seconds {
        Some(remaining_seconds) => format!(
            "{}:{}:{} remaining",
            remaining_seconds / 3600,
            (remaining_seconds % 3600) / 60,
            remaining_seconds % 60
        ),
        None => "Unknown time remaining.".to_string(),
    };
    //let progress_status = battery_remaining_status;

    let mut toast = Toast::new();
    toast.text1(title).text2(message).text3(format!(
        "Battery level: {}%, {battery_remaining_status}",
        battery_report.percentage
    ));
    /*toast.progress(
        "tag",
        "Now Battery Level:",
        &progress_status,
        progress_value,
        &format!("{}%", battery_report.percentage),
    );*/
    toast
        .duration(ToastDuration::Short)
        .action(Action::new("silent for 5 mins", "silent 5 mins", ""))
        .action(Action::new("silent for 10 mins", "silent 10 mins", ""));
    let mode_guard = match crate::runner::MODE.read() {
        Ok(mode) => mode,
        Err(e) => e.into_inner(),
    };
    match input_type {
        NotificationInputType::ModeSelector if !mode_names.is_empty() => {
            toast.input(
                Input::new("mode_selection", InputType::Selection)
                    .with_title("select mode")
                    .with_default_input(if mode_guard.is_empty() {
                        "mode_no_mode".into()
                    } else {
                        format!("mode:{}", mode_guard)
                    }),
            );
            toast.selection(Selection::new("mode_no_mode", "<no mode>"));
            mode_names
                .iter()
                .map(|mode_name| Selection::new(format!("mode:{}", mode_name), mode_name))
                .for_each(|selection| {
                    toast.selection(selection);
                });
            toast.action(
                Action::new("change mode", "change mode", "").with_input_id("mode_selection"),
            );
        }
        NotificationInputType::ModeSelector | NotificationInputType::SilentSpecifiedMinutes => {
            // if modes is empty, apply SilentSpecifiedMinutes
            toast
                .input(
                    Input::new("silent_time", InputType::Text)
                        .with_title("Input silent minites:")
                        .with_default_input("5"),
                )
                .action(
                    Action::new("mins: Keep silent", "silent specified mins", "")
                        .with_input_id("silent_time"),
                )
                .action(Action::new("change mode", "require change mode", ""));
        }
    }
    drop(mode_guard); // for fast unlock RwLockGuard

    toast_manager
        .on_activated(None, move |action| {
            handle_battery_notify_activated_action(action, &notification_action);
        })
        .show(&toast)?;
    Ok(())
}

fn handle_battery_notify_activated_action(
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
                #[allow(clippy::collapsible_if)]
                "require change mode" => {
                    println!("Required change mode");
                    if let Err(e) = crate::notification::mode_change_notify(
                        &crate::notification::NotificationMethod::WinrtToastReborn,
                        Arc::clone(notification_action),
                    ) {
                        if let Ok(mut action_guard) = notification_action.lock() {
                            *action_guard = Some(NotificationAction::Error(e)); // anyway put error
                            drop(action_guard); // explicit drop to ensure the lock is released
                        }
                    }
                    let mut guard = match notification_action.lock() {
                        Ok(guard) => guard,
                        Err(e) => e.into_inner(),
                    };
                    *guard = Some(NotificationAction::RequireChangeMode);
                }
                "change mode" => {
                    if let Some(mode_to_change) = action.values.get("mode_selection") {
                        let mut guard = match notification_action.lock() {
                            Ok(guard) => guard,
                            Err(e) => e.into_inner(),
                        };
                        if mode_to_change == "mode_no_mode" {
                            println!("change to no mode");
                            *guard = Some(NotificationAction::ChangeMode(String::default()));
                        } else {
                            println!(r#"change mode to id="{}""#, mode_to_change);
                            *guard = Some(NotificationAction::ChangeMode(
                                mode_to_change.get(5..).unwrap_or_default().to_string(),
                            ));
                        }
                    } else {
                        println!("No input value found for change mode.");
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

#[hooq(anyhow)]
pub(super) fn mode_change_notify_winrt_toast_reborn(
    notification_action: Arc<Mutex<Option<NotificationAction>>>,
) -> anyhow::Result<()> {
    let toast_manager = ToastManager::new(crate::aumid::AUMID);
    let mut toast = Toast::new();
    let mode_guard = match crate::runner::MODE.read() {
        Ok(mode) => mode,
        Err(e) => e.into_inner(),
    };
    toast
        .text1("Notify Mode Change")
        .duration(ToastDuration::Long)
        .input(
            Input::new("mode_selection", InputType::Selection)
                .with_title("select mode")
                .with_default_input(if mode_guard.is_empty() {
                    "mode_no_mode".into()
                } else {
                    format!("mode:{}", mode_guard)
                }),
        )
        .selection(Selection::new("mode_no_mode", "<no mode>"));
    drop(mode_guard); // for fast unlock RwLockGuard
    crate::runner::MODE_NAMES
        .get()
        .ok_or_else(|| {
            anyhow::Error::msg("MODE_NAMES is not initilized. This can not happen.".red())
        })?
        .iter()
        .map(|mode_name| Selection::new(format!("mode:{}", mode_name), mode_name))
        .for_each(|selection| {
            toast.selection(selection);
        });
    toast.action(Action::new("change mode", "change mode", "").with_input_id("mode_selection"));
    toast_manager
        .on_activated(None, move |action| {
            handle_mode_change_notify_winrt_toast_reborn(action, &notification_action);
        })
        .show(&toast)?;
    Ok(())
}

fn handle_mode_change_notify_winrt_toast_reborn(
    action: Option<winrt_toast_reborn::ActivatedAction>,
    notification_action: &Arc<Mutex<Option<NotificationAction>>>,
) {
    match action {
        Some(action) => {
            let message = format!("Toast activated with action: {}", action.arg);
            println!("{}", message);
            match action.arg.as_str() {
                #[allow(clippy::collapsible_if)]
                "change mode" => {
                    if let Some(mode_to_change) = action.values.get("mode_selection") {
                        let mut guard = match notification_action.lock() {
                            Ok(guard) => guard,
                            Err(e) => e.into_inner(),
                        };
                        if mode_to_change == "mode_no_mode" {
                            println!("change to no mode");
                            *guard = Some(NotificationAction::ChangeMode(String::default()));
                        } else {
                            println!(r#"change mode to id="{}""#, mode_to_change);
                            *guard = Some(NotificationAction::ChangeMode(
                                mode_to_change.get(5..).unwrap_or_default().to_string(),
                            ));
                        }
                    } else {
                        println!("No input value found for change mode.");
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
