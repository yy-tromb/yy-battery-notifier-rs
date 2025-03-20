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

pub fn register_cli(
    toml_settings_path: Option<String>,
    input_mode: bool,
    default_settings: bool,
) -> anyhow::Result<()> {
    let toml_settings_path: String = match toml_settings_path {
        Some(toml_settings_path) => anyhow::Ok(toml_settings_path),
        None => {
            if (!input_mode) & (!default_settings) {
                anyhow::Result::Err(anyhow::anyhow!(
                    "{} If you want to enable input mode, you can use --input",
                    format!(
                        "This session is nor {} and {}.\n\
                        But {}.",
                        "input mode".bold(),
                        "using default_settings".bold(),
                        "not given settings.toml".bold()
                    )
                    .red()
                ))
            } else if input_mode & (!default_settings) {
                let mut toml_settings_path_input = String::with_capacity(256);
                loop {
                    println!("Input path to settings.toml:");
                    match std::io::stdin().read_line(&mut toml_settings_path_input) {
                        Ok(len) => {
                            if len == 0 || toml_settings_path_input.trim().is_empty() {
                                eprintln!("{}", "Given is empty.".red());
                            } else {
                                break;
                            }
                        }
                        Err(e) => eprintln!("{}", e),
                    };
                }
                anyhow::Ok(toml_settings_path_input)
            } else if (!input_mode) & default_settings {
                let default_toml_settings_path = std::env::current_exe()
                    .inspect_err(|_e| eprintln!("{}", "Failed to get current exe".red()))?
                    .with_file_name("default_settings.toml");
                let default_toml_settings_path =
                    default_toml_settings_path.to_str().ok_or_else(|| {
                        anyhow::anyhow!("path to current exe is empty. Unknown error may occured.")
                    })?;
                anyhow::Ok(default_toml_settings_path.to_string())
            } else {
                anyhow::Result::Err(anyhow::anyhow!(
                    "{} If you want to enable input mode, you can use --input",
                    format!(
                        "This session is {} and {}.\n\
                        {}.",
                        "input mode".bold(),
                        "using default_settings".bold(),
                        "Use one option".bold()
                    )
                    .red()
                ))
            }
        }
    }?;
    let toml_settings_path_absolute = std::fs::canonicalize(toml_settings_path.trim())
        .inspect_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!(
                    "{}",
                    format!("file: '{}' was not found.", toml_settings_path).red()
                );
            } else {
                eprintln!("{}", "Unknown error.".red())
            }
        })?;
    let toml_settings_path_absolute = toml_settings_path_absolute.to_str().ok_or_else(|| {
        anyhow::anyhow!("path to current exe is empty. Unknown error may occured.")
    })?;
    println!(
        "Now start register. settings.toml file: '{}'",
        toml_settings_path_absolute //for remove "\\?\" prefix: &toml_settings_path_absolute[4..]
    );
    register(toml_settings_path_absolute.to_string())
}

fn register(toml_settings_path: String) -> anyhow::Result<()> {
    //use windows::ApplicationModel::{StartupTask, StartupTaskState};
    dbg!(&toml_settings_path);

    // registry version
    let current_exe = std::env::current_exe()
        .inspect_err(|_e| eprintln!("{}", "Failed to get current exe".red()))?
        .with_file_name("yy-battery-notifier-rs.exe");
    let current_exe = current_exe.to_str().ok_or_else(|| {
        anyhow::anyhow!("path to current exe is empty. Unknown error may occured.")
    })?;
    let run_cmd = format!(r#""{}" --msgbox -s "{}""#, current_exe, toml_settings_path);
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
    let tx = windows_registry::Transaction::new().inspect_err(|_e| {
        eprintln!(
            "{}",
            format!(
                r"Failed to open registry 'HKEY_CURRENT_USER\{}'. Not Found.",
                REG_STARTUP_APPROVED_KEY
            )
            .red()
        )
    })?;
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
    tx.commit().inspect_err(|_e| {
        eprintln!(
            "{}",
            format!(
                r"Failed to set registry 'HKEY_CURRENT_USER\{}\{}'.",
                REG_STARTUP_APPROVED_KEY, REG_STARTUP_NAME
            )
            .red()
        );
    })?;

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
    let mut reg_app_approved_value: Vec<u8> = vec![0; 260];
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
                r"Failed to set registry 'HKEY_CURRENT_USER\{}\{}'.\n\
                    Except: {:?} \n\
                    But Found {:?}",
                REG_STARTUP_APPROVED_KEY,
                REG_STARTUP_NAME,
                REG_STARTUP_APPROVED_VALUE,
                &reg_app_approved.1[..REG_STARTUP_APPROVED_VALUE.len()]
            )
            .red()
        ));
    }
    println!("{}", "register sucuessed!".green().on_black());
    anyhow::Ok(())
}

pub fn delete() -> anyhow::Result<()> {
    let key = CURRENT_USER.create(REG_STARTUP_KEY).inspect_err(|_e| {
        //ToDo
        eprintln!(
            "{}",
            format!(
                r"Failed to open registry 'HKEY_CURRENT_USER\{}'.",
                REG_STARTUP_KEY
            )
            .red()
        );
    })?;
    key.remove_value(REG_STARTUP_NAME).or_else(|e| {
        if e.code() == E_FILENOTFOUND {
            println!(
                r"Maybe already deleted registry 'HKEY_CURRENT_USER\{}\{}'",
                REG_STARTUP_KEY, REG_STARTUP_NAME
            );
            anyhow::Ok(())
        } else {
            eprintln!(
                "{}",
                format!(
                    r"Failed to delete registry 'HKEY_CURRENT_USER\{}\{}'.",
                    REG_STARTUP_KEY, REG_STARTUP_NAME
                )
                .red()
            );
            anyhow::Result::Err(anyhow::Error::from(e))
        }
    })?;

    //check
    match key.get_string(REG_STARTUP_NAME) {
        Ok(v) => {
            return anyhow::Result::Err(anyhow::anyhow!(
                "{}",
                format!(
                    r"Failed to delete delete registry 'HKEY_CURRENT_USER\{}\{}'. Found: {}",
                    REG_STARTUP_KEY, REG_STARTUP_NAME, v
                )
            ));
        }
        Err(e) => {
            if e.code() == E_FILENOTFOUND {
                //do nothing
            } else {
                eprintln!(
                    "{}",
                    format!(
                        r"Failed to read registry 'HKEY_CURRENT_USER\{}\{}' for strange reason.",
                        REG_STARTUP_KEY, REG_STARTUP_NAME
                    )
                    .red()
                );
                return anyhow::Result::Err(anyhow::Error::from(e));
            }
        }
    }

    //approved
    let key = CURRENT_USER
        .create(REG_STARTUP_APPROVED_KEY)
        .inspect_err(|_e| {
            //ToDo
            eprintln!(
                "{}",
                format!(
                    r"Failed to open registry 'HKEY_CURRENT_USER\{}'.",
                    REG_STARTUP_APPROVED_KEY
                )
                .red()
            );
        })?;
    key.remove_value(REG_STARTUP_NAME).or_else(|e| {
        if e.code() == E_FILENOTFOUND {
            println!(
                r"Maybe already deleted registry 'HKEY_CURRENT_USER\{}\{}'",
                REG_STARTUP_APPROVED_KEY, REG_STARTUP_NAME
            );
            anyhow::Ok(())
        } else {
            eprintln!(
                "{}",
                format!(
                    r"Failed to delete registry 'HKEY_CURRENT_USER\{}\{}'.",
                    REG_STARTUP_APPROVED_KEY, REG_STARTUP_NAME
                )
                .red()
            );
            anyhow::Result::Err(anyhow::Error::from(e))
        }
    })?;

    //approved check
    let mut reg_app_approved_value: Vec<u8> = vec![0; 260];
    match unsafe { key.raw_get_bytes(REG_STARTUP_NAME_PCWSTR, &mut reg_app_approved_value) } {
        Ok(v) => anyhow::Result::Err(anyhow::anyhow!(
            "{}",
            format!(
                r"Failed to delete registry 'HKEY_CURRENT_USER\{}\{}'.\n\
                    Found {:?}",
                REG_STARTUP_APPROVED_KEY,
                REG_STARTUP_NAME,
                &v.1[..REG_STARTUP_APPROVED_VALUE.len()]
            )
            .red()
        )),
        Err(e) => {
            if e.code() == E_FILENOTFOUND {
                println!("{}", "delete sucuessed!".green().on_black());
                anyhow::Ok(())
            } else {
                eprintln!(
                    "{}",
                    format!(
                        r"Failed to read registry 'HKEY_CURRENT_USER\{}\{}' for strange reason.",
                        REG_STARTUP_KEY, REG_STARTUP_NAME
                    )
                    .red()
                );
                anyhow::Result::Err(anyhow::Error::from(e))
            }
        }
    }
}
