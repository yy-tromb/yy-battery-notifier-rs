use anyhow::Context as _;
use colored::Colorize;
use hooq::hooq;

use std::sync::{Arc, LazyLock, Mutex, OnceLock, RwLock};

pub static MODE_NAMES: OnceLock<Vec<String>> = OnceLock::new();
pub static MODE: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new("".into())); // unset value is "" (modeless).

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
    ) -> anyhow::Result<()> {
        if self.settings.select_mode_when_starts {
            crate::notification::mode_change_notify(
                &self.settings.notification_method,
                notification_action,
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

    pub fn run(self) -> anyhow::Result<()> {
        use crate::notification::{NotificationAction, battery_notify};
        let duration = std::time::Duration::from_secs(self.settings.check_interval);
        let mode_names = MODE_NAMES.get();
        MODE_NAMES
            .set(
                self.settings
                    .modes
                    .keys()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>(),
            )
            .map_err(|_| {
                anyhow::Error::msg(
                    format!(
                        "MODE_NAMES is initialized. Found: {:?} \nThis can not be happen.",
                        mode_names
                    )
                    .red(),
                )
            })?;
        let mode_names = MODE_NAMES.get().ok_or_else(|| {
            anyhow::Error::msg("MODE_NAMES is not initilized. This can not happen.".red())
        })?;
        let mode = &*MODE;
        match mode.write() {
            Ok(mut guard) => {
                *guard = self.settings.initial_mode.clone();
            }
            Err(e) => {
                return Err(anyhow::Error::msg(
                    format!(
                        "Failed to lock MODE. Unknown poison error!\n Error: {:?}",
                        e
                    )
                    .red(),
                ));
            }
        };
        let notification_method = &self.settings.notification_method;
        let notification_action: Arc<Mutex<Option<NotificationAction>>> =
            Arc::new(Mutex::new(None));
        self.outset(Arc::clone(&notification_action))?;

        // auto insert below expression and .with_context between Result (example:`~~~()`) and `?;`
        // most function return Result<()>, so if to recovery, using Ok(())
        #[hooq::method(
            .or_else(|e| {
            if self.settings.abort_on_error_except_initialize {
                Err(e).$so_far // inserted anyhow::Result::with_context() as above impl Cli
            } else {
                // ToDo: Handle error gracefully
                let path = $path;
                let line = $line;
                let col = $col;
                let expr = hooq::summary!($source);
                eprintln!("[{path}:{line}:{col}]\n{expr}");
                Ok(())
            }
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
                            300u64.saturating_sub(self.settings.check_interval),
                        ));
                    }
                    NotificationAction::Silent10Mins => {
                        println!("{}", "Silent for 10 minutes action triggered.".yellow());
                        std::thread::sleep(std::time::Duration::from_secs(
                            600u64.saturating_sub(self.settings.check_interval),
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
                                .saturating_mul(60)
                                .saturating_sub(self.settings.check_interval),
                        ));
                    }
                    NotificationAction::RequireChangeMode => {
                        crate::notification::mode_change_notify(
                            notification_method,
                            Arc::clone(&notification_action),
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
                        if mode_names.contains(&*mode.read().unwrap_or_else(|e| e.into_inner())) {
                            match mode.write() {
                                Ok(mut mode_guard) => {
                                    *mode_guard = mode_to_change.clone();
                                    drop(mode_guard); // explicit drop to ensure lock is released
                                }
                                Err(e) => {
                                    eprintln!(
                                        "[{}:{}:{}] mode.lock() in cli::Cli::run\n{}",
                                        file!(),
                                        line!() - 8, // line of notification_action.lock(),
                                        47,          // column of notification_action.lock(),
                                        "poison error of RwLock of mode.".red()
                                    );
                                    eprintln!("ref: {:?}", e.get_ref());
                                    let mut mode_guard = e.into_inner();
                                    *mode_guard = mode_to_change.clone();
                                    drop(mode_guard); // explicit drop to ensure lock is released
                                }
                            }
                        }
                    }
                    NotificationAction::Error(e) => {
                        Err(anyhow::Error::msg(e.to_string()))?;
                        // cannot convert or deref e to anyhow::Error directly
                        // so must do via String
                    }
                }
                *action_guard = None; // clear action for next check
            }
            drop(action_guard); // Release the lock before checking battery
            println!(
                "mode: {:?}",
                mode.read().unwrap_or_else(|e| e.into_inner(),)
            );
            #[hooq::method(
                .with_context(||{
                    let path = $path;
                    let line = $line;
                    let col = $col;
                    let expr = hooq::summary!($source);
                    format!("[{path}:{line}:{col}]\n{expr}")
                })
            )]
            let battery_report = match crate::battery::battery_check() {
                Ok(report) => report,
                Err(e) => {
                    if self.settings.abort_on_error_except_initialize {
                        return Err(e);
                    } else {
                        eprintln!("Error on checking battery:\n{:?}", e);
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
                        mode_names,
                    )
                })?;
            if let Some(mode_setting) = self
                .settings
                .modes
                .get(&*mode.read().unwrap_or_else(|e| e.into_inner()))
            {
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
                            mode_names,
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
