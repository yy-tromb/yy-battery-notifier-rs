use colored::Colorize;
use windows::core::HRESULT;

use windows_registry::CURRENT_USER;
const REG_STARTUP_KEY: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";
const REG_STARTUP_NAME: &str = "yy-tromb.yy-battery-notifier-rs";
const REG_STARTUP_NAME_PCWSTR: windows::core::PCWSTR =
    windows::core::w!("yy-tromb.yy-battery-notifier-rs");
const REG_STARTUP_APPROVED_KEY: &str =
    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run";
const REG_STARTUP_APPROVED_VALUE: [u8; 12] = [
    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
const E_ACCESSDENIED: HRESULT = HRESULT::from_win32(0x80070005_u32);
const E_FILENOTFOUND: HRESULT = HRESULT::from_win32(0x80070002_u32);

fn register(toml_settings_path: String) -> anyhow::Result<()> {
    //use windows::ApplicationModel::{StartupTask, StartupTaskState};
    dbg!(&toml_settings_path);

    // registry version
    let current_exe = std::env::current_exe()
        .inspect_err(|_e| eprintln!("{}", "Failed to get current exe".red()))?;
    let current_exe = current_exe.to_str().ok_or_else(|| {
        anyhow::anyhow!("path to current exe is empty. Unknown error may occured.")
    })?;
    let run_cmd = format!(r#""{}" --gui -s "{}""#, current_exe, toml_settings_path);
    //set
    let key = CURRENT_USER.create(REG_STARTUP_KEY).inspect_err(|_e| {
        eprintln!(
            "{}",
            format!(
                r"Failed to open registry 'HKEY_CURRENT_USER\{}'.",
                REG_STARTUP_KEY
            )
            .red()
        );
    })?;
    key.set_string(REG_STARTUP_NAME, &run_cmd)
        .inspect_err(|_e| {
            eprintln!(
                "{}",
                format!(
                    r"Failed to set registry 'HKEY_CURRENT_USER\{}\{}'.",
                    REG_STARTUP_KEY, REG_STARTUP_NAME
                )
                .red()
            );
        })?;

    //check
    let reg_startup = key.get_string(REG_STARTUP_NAME).inspect_err(|_e| {
        eprintln!(
            "{}",
            format!(
                r"Failed to get registry 'HKEY_CURRENT_USER\{}\{}'.",
                REG_STARTUP_KEY, REG_STARTUP_NAME
            )
            .red()
        );
    })?;
    if reg_startup != run_cmd {
        return anyhow::Result::Err(anyhow::anyhow!(
            "{}",
            format!(
                "Failed to set correct value to registry.\n Except: '{}' but Found: '{}'",
                run_cmd, reg_startup
            )
            .red()
        ));
    }

    // Approve
    match CURRENT_USER
        .options()
        .write()
        .create()
        .open(REG_STARTUP_APPROVED_KEY)
    {
        Ok(key) => key
            .set_bytes(
                REG_STARTUP_NAME,
                windows_registry::Type::Bytes,
                &REG_STARTUP_APPROVED_VALUE,
            )
            .inspect_err(|_e| {
                eprintln!(
                    "{}",
                    format!(
                        r"Failed to set registry 'HKEY_CURRENT_USER\{}\{}'.",
                        REG_STARTUP_APPROVED_KEY, REG_STARTUP_NAME
                    )
                    .red()
                );
            })?,
        Err(error) => {
            eprintln!(
                "{}",
                format!(
                    r"Failed to open registry 'HKEY_CURRENT_USER\{}'. Not Found.",
                    REG_STARTUP_APPROVED_KEY
                )
                .red()
            );
            return anyhow::Result::Err(error.into());
        }
    }

    //approved check
    let key = CURRENT_USER
        .create(REG_STARTUP_APPROVED_KEY)
        .inspect_err(|_e| {
            eprintln!(
                "{}",
                format!(
                    r"Failed to open registry 'HKEY_CURRENT_USER\{}'.",
                    REG_STARTUP_APPROVED_KEY
                )
                .red()
            );
        })?;
    let mut reg_app_approved_value: Vec<u8> = vec![0; 256];
    dbg!(&reg_app_approved_value.len());
    let reg_app_approved = unsafe {
        key.raw_get_bytes(REG_STARTUP_NAME_PCWSTR, &mut reg_app_approved_value)
            .inspect_err(|_e| {
                eprintln!(
                    "{}",
                    format!(
                        r"Failed to get registry 'HKEY_CURRENT_USER\{}\{}'.",
                        REG_STARTUP_APPROVED_KEY, REG_STARTUP_NAME
                    )
                    .red()
                );
            })?
    };
    if reg_app_approved.1 != REG_STARTUP_APPROVED_VALUE {
        return anyhow::Result::Err(anyhow::anyhow!(
            "{}",
            format!(
                "Failed to set correct value to registry. found: '{:?}'",
                reg_app_approved.1
            )
            .red()
        ));
    }
    println!("{}", "register sucuessed!".green().on_black());
    anyhow::Ok(())
}

pub fn register_cli(toml_settings_path: Option<String>, input_mode: bool) -> anyhow::Result<()> {
    use std::path::PathBuf;
    let toml_settings_path_processed: String = match toml_settings_path {
        Some(toml_settings_path) => {
            let toml_settings_metadata =
                std::fs::metadata(&toml_settings_path).inspect_err(|e| {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        eprintln!("{}", format!("{} was not found.", toml_settings_path).red());
                    } else {
                        eprintln!("{}", "Unknown error.".red())
                    }
                })?;
            if !toml_settings_metadata.is_file() {
                return anyhow::Result::Err(anyhow::anyhow!(
                    "Given '{}' is not directory.",
                    toml_settings_path
                ));
            }
            anyhow::Ok(toml_settings_path)
        }
        None => {
            if !input_mode {
                anyhow::Result::Err(anyhow::anyhow!(
                    "{}",
                    format!(
                        "This is not {} But {}.",
                        "input_mode".bold(),
                        "not given settings.toml".bold()
                    )
                    .red()
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
                let toml_settings_path_absolute = std::fs::canonicalize(PathBuf::from(
                    toml_settings_path.trim(),
                ))
                .inspect_err(|e| {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        eprintln!("{}", format!("{} was not found.", toml_settings_path).red());
                    } else {
                        eprintln!("{}", "Unknown error.".red())
                    }
                })?;
                let toml_settings_path_absolute =
                    toml_settings_path_absolute.to_str().ok_or_else(|| {
                        anyhow::anyhow!("path to current exe is empty. Unknown error may occured.")
                    })?;
                println!(
                    "Now start register. settings.toml: '{}'",
                    toml_settings_path_absolute
                );
                anyhow::Ok(toml_settings_path_absolute.to_string())
            }
        }
    }?;
    register(toml_settings_path_processed)
}

pub fn delete() -> anyhow::Result<()> {
    let key = CURRENT_USER.create(REG_STARTUP_KEY).inspect_err(|_e| {
        eprintln!(
            "{}",
            format!(
                r"Failed to open registry 'HKEY_CURRENT_USER\{}'.",
                REG_STARTUP_KEY
            )
            .red()
        );
    })?;
    key.remove_value(REG_STARTUP_NAME).inspect_err(|_e| {
        eprintln!(
            "{}",
            format!(
                r"Failed to remove registry 'HKEY_CURRENT_USER\{}\{}'.",
                REG_STARTUP_KEY, REG_STARTUP_NAME
            )
            .red()
        );
    })?;
    let current_exe = std::env::current_exe()
        .inspect_err(|_e| eprintln!("{}", "Failed to get current exe".red()))?;
    let current_exe = current_exe.to_str().ok_or_else(|| {
        anyhow::anyhow!("oath to current exe is empty. Unknown error may occured.")
    })?;
    let run_cmd_part = format!("{} --gui -s ", current_exe);
    let reg_startup = key.get_string(REG_STARTUP_NAME).inspect_err(|_e| {
        eprintln!(
            "{}",
            format!(
                r"Failed to get registry 'HKEY_CURRENT_USER\{}\{}'.",
                REG_STARTUP_KEY, REG_STARTUP_NAME
            )
            .red()
        );
    })?;
    if reg_startup.starts_with(&run_cmd_part) {
        return anyhow::Result::Err(anyhow::anyhow!(
            "{}",
            format!(
                "Failed to set correct value to registry.\n Except: '{}<Your >' but Found: '{}'",
                run_cmd_part, reg_startup
            )
            .red()
        ));
    }
    anyhow::Ok(())
}
