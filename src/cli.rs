use std::sync::{Arc, Mutex};

use colored::Colorize;

pub struct Cli {
    settings: crate::settings::Settings,
}

impl Cli {
    pub fn new(settings: crate::settings::Settings) -> Self {
        Self { settings }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        use crate::notification::{NotificationAction, battery_notify};
        let duration = std::time::Duration::from_secs(self.settings.check_interval);
        let mode_names: Vec<&String> = self.settings.modes.keys().collect();
        let mut mode = self.settings.default_mode.clone();
        let notification_method = &self.settings.notification_method;
        let notification_action: Arc<Mutex<Option<NotificationAction>>> =
            Arc::new(Mutex::new(None));
        loop {
            let mut action_guard = match notification_action.lock() {
                Ok(action_guard) => action_guard,
                Err(e) => {
                    eprintln!("{}", "Failed to read notification action.".red());
                    eprintln!("{:?}", e.get_ref());
                    e.into_inner()
                }
            };
            if let Some(action) = &*action_guard {
                match action {
                    NotificationAction::Silent5Mins => {
                        println!("{}", "Silent for 5 minutes action triggered.".yellow());
                        std::thread::sleep(std::time::Duration::from_secs(
                            300u64
                                .checked_sub(self.settings.check_interval)
                                .unwrap_or(0),
                        ));
                    }
                    NotificationAction::Silent10Mins => {
                        println!("{}", "Silent for 10 minutes action triggered.".yellow());
                        std::thread::sleep(std::time::Duration::from_secs(
                            600u64
                                .checked_sub(self.settings.check_interval)
                                .unwrap_or(0),
                        ));
                    }
                    NotificationAction::SilentSpecifiedMins(specified_mins) => {
                        println!(
                            "{}",
                            format!("Silent for {} minutes action triggered.", specified_mins)
                                .yellow()
                        );
                        std::thread::sleep(std::time::Duration::from_secs(
                            specified_mins
                                .checked_mul(60)
                                .unwrap_or(u64::MAX)
                                .checked_sub(self.settings.check_interval)
                                .unwrap_or(0),
                        ));
                    }
                    NotificationAction::RequireChangeMode => {
                        todo!();
                    }
                    NotificationAction::ChangeMode(mode_to_change) => {
                        println!(
                            "{}",
                            format!(r#"change mode to "{}" action triggered."#, mode_to_change)
                                .yellow()
                        );
                        if mode_names.contains(&&mode) {
                            mode = mode_to_change.clone();
                        }
                    }
                }
                *action_guard = None; // clear action for next check
            }
            drop(action_guard); // Release the lock before checking battery
            if !mode.is_empty() {
                println!(r#"mode: "{}""#, mode)
            } else {
                println!("no mode");
            };
            let battery_report = crate::battery::battery_check().inspect_err(|_e| {
                eprintln!("{}", "Failed to check battery information.".red());
            })?;
            dbg!(&battery_report);
            self.settings
                .notifications
                .iter()
                .filter(|notification_setting| {
                    crate::notification::judge_notification(notification_setting, &battery_report)
                })
                .try_for_each(|notification_setting| {
                    battery_notify(
                        &battery_report,
                        &notification_setting.title,
                        &notification_setting.message,
                        notification_method,
                        Arc::clone(&notification_action),
                        &mode_names,
                        &mode,
                    )
                })?;
            if let Some(mode_setting) = self.settings.modes.get(&mode) {
                mode_setting
                    .notifications
                    .iter()
                    .filter(|notification_setting| {
                        crate::notification::judge_notification(
                            notification_setting,
                            &battery_report,
                        )
                    })
                    .try_for_each(|notification_setting| {
                        battery_notify(
                            &battery_report,
                            &notification_setting.title,
                            &notification_setting.message,
                            notification_method,
                            Arc::clone(&notification_action),
                            &mode_names,
                            &mode,
                        )
                    })?;
            };
            println!("check battery and notifying");
            std::thread::sleep(duration);
        }
    }
}
