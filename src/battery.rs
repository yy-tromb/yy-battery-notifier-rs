struct BatteryReport {
    percentage: u32,
    seconds: u32,
}

impl BatteryReport {
    fn report_windows_notification(&self) -> anyhow::Result<()> {
        // do something
        anyhow::Ok(())
    }
}

fn battery_check_winrt() -> BatteryReport {
    // do something
    BatteryReport {
        percentage: 0,
        seconds: 0,
    }
}
