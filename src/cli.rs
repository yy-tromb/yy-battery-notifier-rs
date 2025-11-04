use std::sync::{Arc, RwLock};

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
        let notification_action: Arc<RwLock<Option<NotificationAction>>> =
            Arc::new(RwLock::new(None));
        loop {
            match notification_action.read() {
                Ok(action_guard) => {
                    if let Some(action) = &*action_guard {
                        match action {
                            NotificationAction::Silent5Mins => {
                                println!("{}", "Silent for 5 minutes action triggered.".yellow());
                                std::thread::sleep(std::time::Duration::from_secs(300-self.settings.check_interval));
                            }
                            NotificationAction::Silent10Mins => {
                                println!("{}", "Silent for 10 minutes action triggered.".yellow());
                                std::thread::sleep(std::time::Duration::from_secs(600-self.settings.check_interval));
                            }
                        }
                    }
                }
                Err(_e) => {
                    eprintln!("{}", "Failed to read notification action.".red());
                }
            }
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
