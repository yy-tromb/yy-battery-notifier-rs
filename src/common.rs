#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TOMLSettings {
    pub check_interval: u64,
    pub notifications: Vec<NotificationTOMLSetting>,
}

impl TryFrom<TOMLSettings> for Settings {
    type Error = anyhow::Error;
    fn try_from(toml_settings: TOMLSettings) -> anyhow::Result<Self> {
        use colored::Colorize;
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
        Ok(settings)
    }
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub check_interval: u64,
    pub notifications: Vec<NotificationSetting>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct NotificationTOMLSetting {
    pub percentage: String,
    pub power_supply: String,
    pub title: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct NotificationSetting {
    pub percentage: String,
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

pub fn error_to_string(error: &anyhow::Error) -> String {
    //ToDo: error handle system
    let msg = format!("{}", error);
    msg.replace("\x1b[31m", "")
        .replace("\x1b[1m", "")
        .replace("\x1b[0m", "")
}

pub fn msgbox(error: &anyhow::Error) -> anyhow::Result<()> {
    use windows::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MESSAGEBOX_RESULT, MessageBoxW};
    use windows::core::HSTRING;
    const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
    let title = HSTRING::from(CARGO_PKG_NAME);
    let content = HSTRING::from(error_to_string(error));

    match unsafe { MessageBoxW(None, &content, &title, MB_ICONERROR) } {
        MESSAGEBOX_RESULT(0) => {
            anyhow::Result::Err(anyhow::Error::from(windows::core::Error::from_win32()))
        }
        _ => anyhow::Ok(()),
    }
}
