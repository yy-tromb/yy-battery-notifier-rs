use colored::Colorize;

fn register(toml_settings_path: String) -> anyhow::Result<()> {
    //use windows::ApplicationModel::{StartupTask, StartupTaskState};
    dbg!(&toml_settings_path);
    // registry version
    use windows::core::HRESULT;
    use windows_registry::CURRENT_USER;
    const TASK_MANAGER_OVERRIDE_REGKEY: &str =
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run";
    const TASK_MANAGER_OVERRIDE_ENABLED_VALUE: [u8; 12] = [
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    const E_ACCESSDENIED: HRESULT = HRESULT::from_win32(0x80070005_u32);
    const E_FILENOTFOUND: HRESULT = HRESULT::from_win32(0x80070002_u32);
    let key = CURRENT_USER.create(r"Software\Microsoft\Windows\CurrentVersion\Run").inspect_err(|_e| {
            eprintln!("{}", r"Failed to open registry 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run'.".red());
        })?;
    let current_exe = std::env::current_exe()
        .inspect_err(|_e| eprintln!("{}", "Failed to get current exe".red()))?;
    let current_exe = current_exe.to_str().ok_or_else(|| {
        anyhow::anyhow!("oath to current exe is empty. Unknown error may occured.")
    })?;
    let run_cmd = format!("{} -s {}", current_exe, toml_settings_path);
    key.set_string(
        "yy-tromb.yy-battery-notifier-rs",
        &run_cmd,
    ).inspect_err(|_e| {
            eprintln!("{}", r"Failed to set registry 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run\yy-tromb.yy-battery-notifier-rs'.".red());
    })?;
    let reg_app =key.get_string("yy-tromb.yy-battery-notifier-rs").inspect_err(|_e| {
            eprintln!("{}", r"Failed to get registry 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run\yy-tromb.yy-battery-notifier-rs'.".red());
    })?;
    if reg_app != run_cmd {
        return anyhow::Result::Err(anyhow::anyhow!(
            "{}",
            format!(
                "Failed to set correct value to registry. found: '{}'",
                reg_app
            )
            .red()
        ));
    }
    // Approve
    match CURRENT_USER
        .options()
        .write()
        .open(TASK_MANAGER_OVERRIDE_REGKEY)
    {
        Ok(key) => key.set_bytes(
            "yy-tromb.yy-battery-notifier-rs",
            windows_registry::Type::Bytes,
            &TASK_MANAGER_OVERRIDE_ENABLED_VALUE,
        ).inspect_err(|_e| {
            eprintln!("{}", r"Failed to set registry 'HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run\yy-tromb.yy-battery-notifier-rs'.".red());
    })?,
        Err(error) => {
            eprintln!("{}", r"Failed to set registry 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run\yy-tromb.yy-battery-notifier-rs'. Not Found.".red());
            return anyhow::Result::Err(error.into());
        }
    }
    let key = CURRENT_USER.create(&TASK_MANAGER_OVERRIDE_REGKEY).inspect_err(|_e| {
            eprintln!("{}", r"Failed to open registry 'HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run'.".red());
        })?;
    let mut reg_app_approved_value: Vec<u8> = vec![0 ; 256];
    dbg!(&reg_app_approved_value.len());
    let reg_app_approved = unsafe {
        key.raw_get_bytes(windows::core::w!("yy-tromb.yy-battery-notifier-rs"),&mut reg_app_approved_value).inspect_err(|_e| {
            eprintln!("{}", r"Failed to get registry 'HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run\yy-tromb.yy-battery-notifier-rs'.".red());
        })?
    };
    if reg_app_approved.1 != TASK_MANAGER_OVERRIDE_ENABLED_VALUE {
        return anyhow::Result::Err(anyhow::anyhow!(
            "{}",
            format!(
                "Failed to set correct value to registry. found: '{}'",
                reg_app
            )
            .red()
        ));
    }
    println!("{}", "register sucuessed!".green().on_black());
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
    let key = CURRENT_USER.create(r"Software\Microsof
t\Windows\CurrentVersion\Run").inspect_err(|_e| {                eprintln!("{}", r"Failed to open registry 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run'.".red());                                      })?;
    key.remove_value(                                          "yy-tromb.yy-battery-notifier-rs").inspect_err(|_e| {                                         eprintln!("{}", r"Failed to remove  registry 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run\yy-tromb.yy-battery-notifier-rs'.".red());
    })?;
    anyhow::Ok(())
}
