#[derive(Debug, Clone)]
pub struct BatteryReport {
    pub percentage: u32,
    pub remaining_seconds: Option<u64>,
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
    let remaining_seconds = {
        let remaining_100_nano_seconds = PowerManager::RemainingDischargeTime()?.Duration;
        if remaining_100_nano_seconds == i64::MAX {
            None
        } else {
            Some((remaining_100_nano_seconds / 10_000_000) as u64)
        }
    };
    let power_supply: PowerSupply = PowerManager::PowerSupplyStatus()?.into();
    // do something
    Ok(BatteryReport {
        percentage,
        remaining_seconds,
        power_supply,
    })
}
