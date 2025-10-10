#[derive(Debug, Clone)]
pub struct BatteryReport {
    pub percentage: u32,
    pub remaining_seconds: u64,
    pub power_supply: PowerSupply,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowerSupply {
    Adequate,
    InAdequate,
    None,
}

impl From<windows::System::Power::PowerSupplyStatus> for PowerSupply {
    fn from(value: windows::System::Power::PowerSupplyStatus) -> Self {
        use windows::System::Power::PowerSupplyStatus;
        match value {
            PowerSupplyStatus::Adequate => PowerSupply::Adequate,
            PowerSupplyStatus::Inadequate => PowerSupply::InAdequate,
            PowerSupplyStatus::NotPresent => PowerSupply::None,
            _ => unreachable!(),
        }
    }
}

#[inline]
pub fn battery_check() -> anyhow::Result<BatteryReport> {
    battery_check_winrt()
}

fn battery_check_winrt() -> anyhow::Result<BatteryReport> {
    use windows::System::Power::PowerManager;
    let percentage = PowerManager::RemainingChargePercent()? as u32;
    let remaining_seconds = PowerManager::RemainingDischargeTime()?.Duration as u64 / 1000_1000_1000_1000;
    let power_supply: PowerSupply = PowerManager::PowerSupplyStatus()?.into();
    // do something
    Ok(BatteryReport {
        percentage,
        remaining_seconds,
        power_supply,
    })
}
