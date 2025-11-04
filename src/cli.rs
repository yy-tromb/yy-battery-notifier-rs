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
            None => &NotificationMethod::WinrtToastReborn
            
        };
        loop {
            let battery_report = crate::battery::battery_check().inspect_err(|_e| {
                eprintln!("{}", "Failed to check battery information.".red());
            })?;
            dbg!(&battery_report);
            for notification_setting in &self.settings.notifications {
                let notification_action = match notification_setting.percentage_symbol {
                    crate::settings::PercentageSymbol::Excess => {
                        if (battery_report.percentage > notification_setting.percentage_int)
                            && (battery_report.power_supply == notification_setting.power_supply)
                        {
                            notify(
                                &battery_report,
                                &notification_setting.title,
                                &notification_setting.message,
                                notification_method,
                            )?
                        } else {
                            None
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
                            )?
                        } else {
                            None
                        }
                    }
                };
                if let Some(action) = notification_action {
                    match action {
                        NotificationAction::Temporary1 => {
                            println!("Temporary action 1 executed from CLI.");
                        }
                        NotificationAction::Temporary2 => {
                            println!("Temporary action 2 executed from CLI.");
                        }
                    }
                }
            }
            println!("check battery and notifying");
            std::thread::sleep(duration);
        }
    }
}
