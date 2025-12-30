use anyhow::Context as _;
use colored::Colorize;
use hooq::hooq;

use std::sync::{Arc, Mutex};

pub struct Cli {
    settings: crate::settings::Settings,
}

// auto insert .with_context() between Result (example: func()->Result<>:`func()`) and `?;`
#[hooq(anyhow)]
impl Cli {
    pub fn new(settings: crate::settings::Settings) -> Self {
        Self { settings }
    }

    #[inline]
    fn outset(
        &self,
        notification_action: Arc<Mutex<Option<crate::notification::NotificationAction>>>,
        mode_names: &[&String],
    ) -> anyhow::Result<()> {
        if self.settings.select_mode_when_starts {
            crate::notification::mode_change_notify(
                &self.settings.notification_method,
                notification_action,
                mode_names,
                &self.settings.initial_mode,
            )?;
        }
        if let Some(wait_time) = self
            .settings
            .wait_seconds_after_select_mode_notify_when_starts
        {
            std::thread::sleep(std::time::Duration::from_secs(wait_time));
        }
        Ok(())
    }

    pub fn run(&self) -> anyhow::Result<()> {
        use crate::notification::{NotificationAction, battery_notify};
        let duration = std::time::Duration::from_secs(self.settings.check_interval);
        let mode_names: Vec<&String> = self.settings.modes.keys().collect();
        let mut mode = self.settings.initial_mode.clone();
        let notification_method = &self.settings.notification_method;
        let notification_action: Arc<Mutex<Option<NotificationAction>>> =
            Arc::new(Mutex::new(None));
        self.outset(Arc::clone(&notification_action), &mode_names)?;

        // auto insert below expression and .with_context between Result (example:`~~~()`) and `?;`
        // most function return Result<()>, so if to recovery, using Ok(())
        #[hooq::method(.or_else(|e| {
            if self.settings.abort_on_error_except_initialize {
                Err(e)
            } else {
                // ToDo: Handle error gracefully
                let path = $path;
                let line = $line;
                let col = $col;
                let expr = hooq::summary!($source);
                eprintln!("[{path}:{line}:{col}]\n{expr}");
                Ok(())
            }
            .$so_far // inserted anyhow::Result::with_context() as above impl Cli
        }))]
        loop {
            let mut action_guard = match notification_action.lock() {
                Ok(action_guard) => action_guard,
                Err(e) => {
                    eprintln!(
                        "[{}:{}:{}] notification_action.lock() in cli::Cli::run\n{}",
                        file!(),
                        line!() - 6, // line of notification_action.lock(),
                        68,          // column of notification_action.lock(),
                        "poison error of Mutex of notification action.".red()
                    );
                    eprintln!("ref: {:?}", e.get_ref());
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
                        crate::notification::mode_change_notify(
                            notification_method,
                            Arc::clone(&notification_action),
                            &mode_names,
                            &mode,
                        )?;
                        if !self.settings.notify_battery_during_change_mode {
                            // do not clear action for user to miss dismiss change mode notification
                            drop(action_guard);
                            std::thread::sleep(duration);
                            continue;
                        }
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
            let battery_report = match crate::battery::battery_check() {
                Ok(report) => report,
                Err(e) => {
                    if self.settings.abort_on_error_except_initialize {
                        return Err(e);
                    } else {
                        eprintln!("Error checking battery: {}", e);
                        std::thread::sleep(duration);
                        continue; // ToDo: Handle error gracefully
                    }
                }
            };
            dbg!(&battery_report);
            self.settings
                .notifications
                .iter()
                .filter(|notification_setting| {
                    crate::notification::judge_notification(notification_setting, &battery_report)
                })
                .try_for_each(|notification_setting| {
                    battery_notify(
                        notification_method,
                        &battery_report,
                        &notification_setting.title,
                        &notification_setting.message,
                        Arc::clone(&notification_action),
                        &notification_setting.input_type,
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
                            notification_method,
                            &battery_report,
                            &notification_setting.title,
                            &notification_setting.message,
                            Arc::clone(&notification_action),
                            &notification_setting.input_type,
                            &mode_names,
                            &mode,
                        )
                    })?;
            };
            println!("check battery and notifying");
            std::thread::sleep(duration);
        }
        #[allow(unreachable_code)]
        Ok(())
    }
}
