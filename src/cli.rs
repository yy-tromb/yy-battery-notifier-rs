use colored::Colorize;

pub struct Cli {
    settings: crate::common::Settings,
}

impl Cli {
    pub fn new(toml_settings: crate::common::TOMLSettings) -> anyhow::Result<Self> {
        let mut settings = crate::common::Settings {
            check_interval: toml_settings.check_interval,
            notifications: Vec::with_capacity(toml_settings.notifications.len()),
        };
        for notification_toml_setting in toml_settings.notifications {
            let percentage = notification_toml_setting.percentage;
            let Some(percentage_symbol) = percentage.chars().last() else {
                eprintln!(
                    "{}",
                    format!("percentage may be empty. found:'{}'.", &percentage).red()
                );
                return Err(anyhow::anyhow!(
                    "percentage may be empty. found:'{}'.",
                    &percentage
                ));
            };
            let percentage_symbol = match percentage_symbol {
                '+' => crate::common::PercentageSymbol::Excess,
                '-' => crate::common::PercentageSymbol::Under,
                _ => {
                    eprintln!(
                        "{}",
                        format!(
                            "Failed to interpret '{}' as percentage symbol.",
                            &percentage_symbol
                        )
                        .red()
                    );
                    return Err(anyhow::anyhow!(
                        "Failed to interpret '{}' as percentage symbol.",
                        &percentage_symbol
                    ));
                }
            };
            let percentage_int: u32 =
                percentage[0..percentage.len() - 1]
                    .parse()
                    .inspect_err(|_e| {
                        eprintln!(
                            "{}",
                            format!("Failed to interpret '{}' as percentage value.", &percentage)
                                .red()
                        );
                    })?;
            let power_supply: crate::battery::PowerSupply = match notification_toml_setting
                .power_supply
                .as_str()
            {
                "Adequate" => crate::battery::PowerSupply::Adequate,
                "InAdequate" => crate::battery::PowerSupply::InAdequate,
                "None" => crate::battery::PowerSupply::None,
                _ => {
                    eprintln!(
                            "{}",
                            format!(
                                r#"Failed to interpret power_supply:'{}'. Use "Adequate" , "InAdequate" or "None"."#,
                                &notification_toml_setting.power_supply
                            )
                            .red()
                        );
                    return Err(anyhow::anyhow!(
                        r#"Failed to interpret power_supply:'{}'. Use "Adequate" , "InAdequate" or "None"."#,
                        &notification_toml_setting.power_supply
                    ));
                }
            };
            settings
                .notifications
                .push(crate::common::NotificationSetting {
                    percentage,
                    percentage_int,
                    percentage_symbol,
                    power_supply,
                    title: notification_toml_setting.title,
                    message: notification_toml_setting.message,
                });
        }
        dbg!(&settings);
        Ok(Self { settings })
    }

    pub fn start(&self) -> anyhow::Result<()> {
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
