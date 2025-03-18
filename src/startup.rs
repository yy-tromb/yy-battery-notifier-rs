use colored::Colorize;

fn register(toml_settings_path: String) -> anyhow::Result<()> {
    //use windows::ApplicationModel::{StartupTask, StartupTaskState};
    dbg!(&toml_settings_path);
    // registry version
    use windows_registry::CURRENT_USER;
    let key = CURRENT_USER.create(r"Software\Microsoft\Windows\CurrentVersion\Run").inspect_err(|_e| {
            eprintln!("{}", r"Failed to open registry 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run'.".red());
        })?;
    let current_exe = std::env::current_exe()
        .inspect_err(|_e| eprintln!("{}", "Failed to get current exe".red()))?;
    let current_exe = current_exe.to_str().ok_or_else(|| {
        anyhow::anyhow!("oath to current exe is empty. Unknown error may occured.")
    })?;
    key.set_string(
        "yy-tromb.yy-battery-notifier-rs",
        format!("{} -s {}", current_exe, toml_settings_path),
    ).inspect_err(|_e| {
            eprintln!("{}", r"Failed to set registry 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run\yy-tromb.yy-battery-notifier-rs'.".red());
    })?;

    anyhow::Ok(())
}

pub fn register_cli(toml_settings_path: Option<String>, input_mode: bool) -> anyhow::Result<()> {
    let toml_settings_path: String = match toml_settings_path {
        Some(toml_settings_path) => anyhow::Ok(toml_settings_path),
        None => {
            if !input_mode {
                anyhow::Result::Err(anyhow::anyhow!(
                    "{}",
                    "input_mode is false but not given settings.toml.".red()
                ))
            } else {
                let mut toml_settings_path = String::with_capacity(256);
                loop {
                    println!("Input path to settings.toml:");
                    match std::io::stdin().read_line(&mut toml_settings_path) {
                        Ok(len) => {
                            if len == 0 || toml_settings_path.trim().is_empty() {
                                eprintln!("{}", "Given is empty.".red());
                            } else {
                                break;
                            }
                        }
                        Err(e) => eprintln!("{}", e),
                    };
                }
                anyhow::Ok(toml_settings_path.trim().to_string())
            }
        }
    }?;
    register(toml_settings_path)
}

pub fn delete() -> anyhow::Result<()> {
    anyhow::Ok(())
}
