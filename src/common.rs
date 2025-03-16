#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TOMLSettings {
    pub check_interval: u64,
    pub notifications: Vec<NotificationTOMLSetting>,
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
