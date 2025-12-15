use colored::Colorize;
use std::collections::HashMap;

use crate::notification::NotificationMethod;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TOMLSettings {
    pub check_interval: u64,
    pub notifications: Vec<NotificationTOMLSetting>,
    pub notification_method: Option<NotificationMethod>,
    pub modes: Option<HashMap<String, NotificationTOMLSetting>>,
    pub default_mode: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct NotificationTOMLSetting {
    pub percentage: String,
    pub power_supply: String,
    pub title: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub check_interval: u64,
    pub notifications: Vec<NotificationSetting>,
    pub notification_method: NotificationMethod,
    pub modes: std::collections::HashMap<String, NotificationSetting>,
    pub default_mode: String,
}

#[derive(Debug, Clone)]
pub struct NotificationSetting {
    pub percentage_int: u32,
    pub percentage_symbol: PercentageSymbol,
    pub power_supply: crate::battery::PowerSupply,
    pub title: String,
    pub message: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum PercentageSymbol {
    Excess,
    Under,
}

impl TryFrom<TOMLSettings> for Settings {
    type Error = anyhow::Error;
    fn try_from(toml_settings: TOMLSettings) -> anyhow::Result<Self> {
        let mut settings = Settings {
            check_interval: toml_settings.check_interval,
            notifications: Vec::with_capacity(toml_settings.notifications.len()),
            notification_method: toml_settings.notification_method.unwrap_or_default(),
            modes: HashMap::new(),
            default_mode: toml_settings.default_mode.unwrap_or_default(),
        };
        for notification_toml_setting in toml_settings.notifications {
            settings
                .notifications
                .push(notification_toml_setting.try_into()?);
        }
        for (mode, notification_toml_setting) in
            toml_settings.modes.unwrap_or_else(|| HashMap::new())
        {
            settings
                .modes
                .insert(mode, notification_toml_setting.try_into()?);
        }
        dbg!(&settings);
        Ok(settings)
    }
}

impl TryFrom<NotificationTOMLSetting> for NotificationSetting {
    type Error = anyhow::Error;
    fn try_from(notification_toml_setting: NotificationTOMLSetting) -> anyhow::Result<Self> {
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
            '+' => PercentageSymbol::Excess,
            '-' => PercentageSymbol::Under,
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
                        format!("Failed to interpret '{}' as percentage value.", &percentage).red()
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
        Ok(NotificationSetting {
            percentage_int,
            percentage_symbol,
            power_supply,
            title: notification_toml_setting.title,
            message: notification_toml_setting.message,
        })
    }
}
