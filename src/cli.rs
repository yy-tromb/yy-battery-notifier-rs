use colored::Colorize;

pub struct Cli {
    settings: crate::settings::Settings,
}

impl Cli {
    pub fn new(settings: crate::settings::Settings) -> anyhow::Result<Self> {
        Ok(Self { settings })
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let duration = std::time::Duration::from_secs(self.settings.check_interval);
        loop {
            let battery_report = crate::battery::battery_check().inspect_err(|_e| {
                eprintln!("{}", "Failed to check battery information.".red());
            })?;
            for notification_setting in &self.settings.notifications {
                match notification_setting.percentage_symbol {
                    crate::common::PercentageSymbol::Excess => {
                        if (battery_report.percentage > notification_setting.percentage_int)
                            && (battery_report.power_supply == notification_setting.power_supply)
                        {
                            crate::notify::notify(
                                &battery_report,
                                &notification_setting.title,
                                &notification_setting.message,
                            )?
                        }
                    }
                    crate::common::PercentageSymbol::Under => {
                        if (battery_report.percentage < notification_setting.percentage_int)
                            && (battery_report.power_supply == notification_setting.power_supply)
                        {
                            crate::notify::notify(
                                &battery_report,
                                &notification_setting.title,
                                &notification_setting.message,
                            )?
                        }
                    }
                }
            }
            println!("check battery and notifying");
            std::thread::sleep(duration);
        }
    }
}
