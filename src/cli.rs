use std::sync::{Arc, RwLock,Mutex};

use colored::Colorize;

pub struct Cli {
    settings: crate::settings::Settings,
}

impl Cli {
    pub fn new(settings: crate::settings::Settings) -> anyhow::Result<Self> {
        Ok(Self { settings })
    }

    pub fn run(&self) -> anyhow::Result<()> {
        use crate::notification::{NotificationAction, NotificationMethod, notify};
        let duration = std::time::Duration::from_secs(self.settings.check_interval);
        let notification_method = match &self.settings.notification_method {
            Some(method) => method,
            None => &NotificationMethod::TauriWinrtToast,
        };
        let notification_action: Arc<Mutex<Option<NotificationAction>>> =
            Arc::new(Mutex::new(None));
        loop {
            let action_guard = match notification_action.lock() {
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
                            300 - self.settings.check_interval,
                        ));
                    }
                    NotificationAction::Silent10Mins => {
                        println!("{}", "Silent for 10 minutes action triggered.".yellow());
                        std::thread::sleep(std::time::Duration::from_secs(
                            600 - self.settings.check_interval,
                        ));
                    }
                    NotificationAction::SilentSpecifiedMins(specified_mins)=> {
                        println!("{}", format!("Silent for {} minutes action triggered.", specified_mins).yellow());
                        std::thread::sleep(std::time::Duration::from_secs(
                            specified_mins.checked_mul(60).unwrap_or(600) - self.settings.check_interval,
                        ));
                    }
                }
            }
            drop(action_guard); // Release the lock before checking battery
            let battery_report = crate::battery::battery_check().inspect_err(|_e| {
                eprintln!("{}", "Failed to check battery information.".red());
            })?;
            dbg!(&battery_report);
            for notification_setting in &self.settings.notifications {
                match notification_setting.percentage_symbol {
                    crate::settings::PercentageSymbol::Excess => {
                        if (battery_report.percentage > notification_setting.percentage_int)
                            && (battery_report.power_supply == notification_setting.power_supply)
                        {
                            notify(
                                &battery_report,
                                &notification_setting.title,
                                &notification_setting.message,
                                notification_method,
                                notification_action.clone(),
                            )?
                        }
                    }
                    crate::settings::PercentageSymbol::Under => {
                        if (battery_report.percentage < notification_setting.percentage_int)
                            && (battery_report.power_supply == notification_setting.power_supply)
                        {
                            notify(
                                &battery_report,
                                &notification_setting.title,
                                &notification_setting.message,
                                notification_method,
                                notification_action.clone(),
                            )?
                        }
                    }
                };
            }
            println!("check battery and notifying");
            std::thread::sleep(duration);
        }
    }
}
