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

pub fn battery_check() -> anyhow::Result<BatteryReport> {
    let mut battery_report = battery_check_winrt()?;
    dbg!(&battery_report);
    // temporary
    if battery_report.remaining_seconds.is_none() {
        let batttery_report_win32 = battery_check_win32()?;
        dbg!(&batttery_report_win32);
        if batttery_report_win32.remaining_seconds.is_some() {
            battery_report.remaining_seconds = batttery_report_win32.remaining_seconds;
        }
        Ok(battery_report)
    } else {
        Ok(battery_report)
    }
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

fn battery_check_win32() -> anyhow::Result<BatteryReport> {
    use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};
    let mut system_power_status = SYSTEM_POWER_STATUS::default();
    unsafe {
        GetSystemPowerStatus(&mut system_power_status)?;
    }
    let percentage = system_power_status.BatteryLifePercent as u32;
    let remaining_seconds = if system_power_status.BatteryLifeTime == u32::MAX {
        None
    } else {
        Some(system_power_status.BatteryLifeTime as u64)
    };
    let power_supply = match system_power_status.ACLineStatus {
        0 => PowerSupply::None,
        1 => PowerSupply::Adequate,
        255 => PowerSupply::InAdequate,
        _ => unreachable!(),
    };
    Ok(BatteryReport {
        percentage,
        remaining_seconds,
        power_supply,
    })
}
