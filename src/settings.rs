use anyhow::Context as _;
use colored::Colorize;
use hooq::hooq;
use rustc_hash::{FxBuildHasher, FxHashMap};

type FxIndexMap<K, V> = indexmap::IndexMap<K, V, FxBuildHasher>;

use crate::notification::NotificationInputType;
use crate::notification::NotificationMethod;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TOMLSettings {
    pub check_interval: u64,
    pub notification_method: Option<NotificationMethod>,
    pub mode_names: Option<Vec<String>>,
    pub initial_mode: Option<String>,
    pub abort_on_error_except_initialize: Option<bool>,
    pub notify_battery_during_change_mode: Option<bool>,
    pub select_mode_when_starts: Option<bool>,
    pub wait_seconds_after_select_mode_notify_when_starts: Option<u64>,
    pub notifications: Option<Vec<NotificationTOMLSetting>>,
    pub modes: Option<FxHashMap<String, ModeTOMLSetting>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct NotificationTOMLSetting {
    pub percentage: String,
    pub power_supply: String,
    pub title: String,
    pub message: String,
    pub input_type: Option<NotificationInputType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ModeTOMLSetting {
    pub notifications: Vec<NotificationTOMLSetting>,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub check_interval: u64,
    pub notification_method: NotificationMethod,
    pub initial_mode: String,
    pub abort_on_error_except_initialize: bool,
    pub notify_battery_during_change_mode: bool,
    pub select_mode_when_starts: bool,
    pub wait_seconds_after_select_mode_notify_when_starts: Option<u64>,
    pub notifications: Vec<NotificationSetting>,
    pub modes: FxIndexMap<String, ModeSetting>,
}

#[derive(Debug, Clone)]
pub struct NotificationSetting {
    pub percentage_int: u32,
    pub percentage_symbol: PercentageSymbol,
    pub power_supply: crate::battery::PowerSupply,
    pub title: String,
    pub message: String,
    pub input_type: NotificationInputType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum PercentageSymbol {
    Excess,
    Under,
}

#[derive(Debug, Clone)]
pub struct ModeSetting {
    pub notifications: Vec<NotificationSetting>,
}

// auto insert .with_context() between Result (example: func()->Result<>:`func()`) and `?;`
#[hooq(anyhow)]
impl TryFrom<TOMLSettings> for Settings {
    type Error = anyhow::Error;
    fn try_from(toml_settings: TOMLSettings) -> anyhow::Result<Self> {
        let mut settings = Settings {
            check_interval: toml_settings.check_interval,
            notification_method: toml_settings.notification_method.unwrap_or_default(),
            initial_mode: toml_settings
                .initial_mode
                .map_or_else(String::default, |initial_mode| {
                    if let Some(modes) = toml_settings.modes.as_ref() {
                        if modes.keys().any(|key| key == &initial_mode) {
                            initial_mode
                        } else {
                            String::default()
                        }
                    } else {
                        String::default()
                    }
                }),
            abort_on_error_except_initialize: toml_settings
                .abort_on_error_except_initialize
                .unwrap_or(false),
            notify_battery_during_change_mode: toml_settings
                .notify_battery_during_change_mode
                .unwrap_or(false),
            select_mode_when_starts: toml_settings.select_mode_when_starts.unwrap_or(true),
            wait_seconds_after_select_mode_notify_when_starts: match toml_settings
                .wait_seconds_after_select_mode_notify_when_starts
            {
                Some(0) => None, // do not wait
                Some(seconds) => Some(seconds),
                None => Some(10), // default seconds
            },
            notifications: Vec::with_capacity(
                toml_settings
                    .notifications
                    .as_ref()
                    .map_or(0usize, |notifications| notifications.len()),
            ),
            modes: FxIndexMap::with_capacity_and_hasher(
                toml_settings
                    .modes
                    .as_ref()
                    .map_or(0usize, |modes| modes.len()),
                Default::default(),
            ),
        };
        for notification_toml_setting in toml_settings.notifications.unwrap_or_default() {
            settings
                .notifications
                .push(notification_toml_setting.try_into()?);
        }
        if let Some(modes) = toml_settings.modes {
            if let Some(mode_names) = toml_settings.mode_names.as_ref() {
                for mode_name in mode_names {
                    if let Some(mode_toml_setting) = modes.get(mode_name) {
                        settings
                            .modes
                            .insert(mode_name.clone(), mode_toml_setting.to_owned().try_into()?);
                    }
                }
            }
            for (mode, mode_toml_setting) in modes.iter().filter(|(key, _)| {
                if let Some(mode_names) = toml_settings.mode_names.as_ref() {
                    !mode_names.contains(key)
                } else {
                    true
                }
            }) {
                settings
                    .modes
                    .insert(mode.clone(), mode_toml_setting.to_owned().try_into()?);
            }
        }

        dbg!(&settings);
        Ok(settings)
    }
}

// auto insert .with_context() between Result (example: func()->Result<>:`func()`) and `?;`
#[hooq(anyhow)]
impl TryFrom<ModeTOMLSetting> for ModeSetting {
    type Error = anyhow::Error;
    fn try_from(mode_toml_setting: ModeTOMLSetting) -> anyhow::Result<Self> {
        let mut mode_settings = Self {
            notifications: Vec::with_capacity(mode_toml_setting.notifications.len()),
        };
        for notification_setting in mode_toml_setting.notifications {
            mode_settings
                .notifications
                .push(notification_setting.try_into()?);
        }
        Ok(mode_settings)
    }
}

// auto insert .with_context() between Result (example: func()->Result<>:`func()`) and `?;`
#[hooq(anyhow)]
impl TryFrom<NotificationTOMLSetting> for NotificationSetting {
    type Error = anyhow::Error;
    fn try_from(notification_toml_setting: NotificationTOMLSetting) -> anyhow::Result<Self> {
        let percentage = notification_toml_setting.percentage;
        let Some(percentage_symbol) = percentage.chars().last() else {
            return Err(anyhow::Error::msg(
                format!("percentage may be empty. found:'{}'.", &percentage).red(),
            ));
        };
        let percentage_symbol = match percentage_symbol {
            '+' => PercentageSymbol::Excess,
            '-' => PercentageSymbol::Under,
            _ => {
                return Err(anyhow::Error::msg(
                    format!(
                        "Failed to interpret '{}' as percentage symbol.",
                        &percentage_symbol
                    )
                    .red(),
                ));
            }
        };
        let percentage_int: u32 =
            percentage[0..percentage.len() - 1]
                .parse()
                .with_context(|| {
                    format!("Failed to interpret '{}' as percentage value.", &percentage).red()
                })?;
        let power_supply: crate::battery::PowerSupply = match notification_toml_setting
            .power_supply
            .as_str()
        {
            "Adequate" => crate::battery::PowerSupply::Adequate,
            "InAdequate" => crate::battery::PowerSupply::InAdequate,
            "None" => crate::battery::PowerSupply::None,
            _ => {
                return Err(anyhow::Error::msg(format!(
                    r#"Failed to interpret power_supply:'{}'. Use "Adequate" , "InAdequate" or "None"."#,
                    &notification_toml_setting.power_supply
                ).red()));
            }
        };
        Ok(NotificationSetting {
            percentage_int,
            percentage_symbol,
            power_supply,
            title: notification_toml_setting.title,
            message: notification_toml_setting.message,
            input_type: notification_toml_setting.input_type.unwrap_or_default(),
        })
    }
}
