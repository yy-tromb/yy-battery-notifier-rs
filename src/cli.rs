pub struct Cli {
    settings: crate::common::Settings,
}

impl Cli {
    pub fn new(settings: crate::common::Settings) -> Self {
        Self { settings }
    }

    pub fn start(&self) -> anyhow::Result<()> {
        let duration = std::time::Duration::from_secs(self.settings.check_interval);
        loop {
            //
            println!("do battery check and notify");
            std::thread::sleep(duration);
        }
    }
}
