#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Settings {
    pub check_interval: u64,
    pub notifications: Vec<NotificationSetting>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct NotificationSetting {
    pub percentage: i32,
    pub message: String,
}
