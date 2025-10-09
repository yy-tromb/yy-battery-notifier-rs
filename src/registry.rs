use colored::Colorize;
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

enum RegistryValue {
    String(String),
    Bytes(Vec<u8>),
}

fn str_to_wide(s: &str) -> windows::core::PCWSTR {
    windows::core::PCWSTR::from_raw(s.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>().as_ptr())
}

pub fn register(
    root: &windows_registry::Key,
    path: &str,
    keys_and_values: &[(&str, &RegistryValue)],
) -> anyhow::Result<()> {
    let tree = root
        .create(path)
        .inspect_err(|_e| {
            if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                eprintln!("{}", format!(r"Failed to open registry 'HKEY_LOCAL_MACHINE\{}'. You may need to run as administrator.",path).red());
            } else {
                eprintln!("{}", format!(r"Failed to open registry 'HKEY_CURRENT_USER\{}'.",path).red());
            }
        })?;
    for (key, value) in keys_and_values {
        match value {
            RegistryValue::String(value_string) => {
                tree.set_string(key, value_string).inspect_err(|_e| {
                    if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                        eprintln!(
                            "{}",
                            format!(
                                r"Failed to set registry 'HKEY_LOCAL_MACHINE\{}\{}'.",
                                path, key
                            )
                            .red()
                        );
                    } else {
                        eprintln!(
                            "{}",
                            format!(
                                r"Failed to set registry 'HKEY_CURRENT_USER\{}\{}'.",
                                path, key
                            )
                            .red()
                        );
                    }
                })?;
            }
            RegistryValue::Bytes(value_bytes) => {
                tree.set_bytes(key, windows_registry::Type::Bytes, value_bytes)
                    .inspect_err(|_e| {
                        if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                            eprintln!(
                                "{}",
                                format!(
                                    r"Failed to set registry 'HKEY_LOCAL_MACHINE\{}\{}'.",
                                    path, key
                                )
                                .red()
                            );
                        } else {
                            eprintln!(
                                "{}",
                                format!(
                                    r"Failed to set registry 'HKEY_CURRENT_USER\{}\{}'.",
                                    path, key
                                )
                                .red()
                            );
                        }
                    })?;
            }
        }
    }
    anyhow::Ok(())
}

fn check(
    root: &windows_registry::Key,
    path: &str,
    keys_and_values: &[(&str, &RegistryValue)],
) -> anyhow::Result<()> {
    let tree = root
        .create(path)
        .inspect_err(|_e| {
            if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                eprintln!("{}", format!(r"Failed to open registry 'HKEY_LOCAL_MACHINE\{}'. You may need to run as administrator.",path).red());
            } else {
                eprintln!("{}", format!(r"Failed to open registry 'HKEY_CURRENT_USER\{}'.",path).red());
            }
        })?;
    for (key, value) in keys_and_values {
        match value {
            RegistryValue::String(value_string) => {
                let read_value = tree.get_string(key).inspect_err(|_e| {
                    if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                        eprintln!(
                            "{}",
                            format!(
                                r"Failed to read registry 'HKEY_LOCAL_MACHINE\{}\{}'.",
                                path, key
                            )
                            .red()
                        );
                    } else {
                        eprintln!(
                            "{}",
                            format!(
                                r"Failed to read registry 'HKEY_CURRENT_USER\{}\{}'.",
                                path, key
                            )
                            .red()
                        );
                    }
                })?;
                if &read_value != value_string {
                    return anyhow::Result::Err(anyhow::anyhow!(format!(
                        r"Registry 'HKEY_CURRENT_USER\{}\{}' has unexpected value. Expected: {}, Found: {}",
                        path, key, value_string, read_value
                    )));
                }
            }
            RegistryValue::Bytes(value_bytes) => {
                let mut buffer: Vec<u8> = vec![0; 260];
                let key_wide = str_to_wide(key);
                let read_value_info = unsafe {
                    tree.raw_get_bytes(&key_wide, &mut buffer).inspect_err(|_e| {
                        if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                            eprintln!(
                                "{}",
                                format!(
                                    r"Failed to read registry 'HKEY_LOCAL_MACHINE\{}\{}'.",
                                    path, key
                                )
                                .red()
                            );
                        } else {
                            eprintln!(
                                "{}",
                                format!(
                                    r"Failed to read registry 'HKEY_CURRENT_USER\{}\{}'.",
                                    path, key
                                )
                                .red()
                            );
                        }
                    })?
                };
                if read_value_info.0 != windows_registry::Type::Bytes
                    || read_value_info.1 != value_bytes.as_slice()
                {
                    if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                        return anyhow::Result::Err(anyhow::anyhow!(
                            "{}",
                            format!(
                                r"'HKEY_LOCAL_MACHINE\{}\{}' has unexpected value.\n\
                    Except: {:?} \n\
                    But Found {:?}",
                                path,
                                key,
                                value_bytes,
                                &read_value_info.1
                            )
                            .red()
                        ));
                    } else {
                        return anyhow::Result::Err(anyhow::anyhow!(
                            "{}",
                            format!(
                                r"'HKEY_CURRENT_USER\{}\{}' has unexpected value.\n\
                    Except: {:?} \n\
                    But Found {:?}",
                                path,
                                key,
                                value_bytes,
                                &read_value_info.1
                            )
                            .red()
                        ));
                    }
                }
            }
        }
    }
    anyhow::Ok(())
}

pub fn register_aumid() -> anyhow::Result<()> {
    let key = CURRENT_USER
        .create(r"SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs")
        .inspect_err(|_e| {
            eprintln!("{}", r"Failed to open registry 'HKEY_CURRENT_USER\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs'.".red());
        })?;
    key.set_string("DisplayName", "yy-battery-notifier-rs").inspect_err(|_e|{
            eprintln!("{}", r"Failed to set registry 'HKEY_CURRENT_USER\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\DisplayName'.".red());
        })?;
    let icon_uri = std::env::current_exe()
        .inspect_err(|_e| eprintln!("{}", "Failed to get current exe".red()))?;
    let icon_uri = icon_uri
        .to_str()
        .unwrap_or(r"C:\Program Files\yy-tromb\yy-battery-notifier-rs\\yy-battery-notifier-rs.exe");
    key.set_string("IconUri", icon_uri).inspect_err(|_e|{
            eprintln!("{}", r"Failed to set registry 'HKEY_CURRENT_USER\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\IconUri'.".red());
        })?;

    //check
    let reg_display_name = key.get_string("DisplayName").inspect_err(|_e| {
        eprintln!("{}", r"Failed to read registry 'HKEY_CURRENT_USER\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\DisplayName'.".red());
    })?;
    let reg_icon_uri = key.get_string("IconUri").inspect_err(|_e| {
        eprintln!("{}", r"Failed to read registry 'HKEY_CURRENT_USER\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\IconUri'.".red());
    })?;
    if &reg_display_name == "yy-battery-notifier-rs" && reg_icon_uri == icon_uri {
        println!("{}", "register sucuessed!".green().on_black());
        anyhow::Ok(())
    } else {
        anyhow::Result::Err(anyhow::anyhow!("Failed to set correct string to registry."))
    }
}

pub fn delete_aumid() -> anyhow::Result<()> {
    let key = CURRENT_USER
        .create(r"SOFTWARE\Classes\AppUserModelId")
        .inspect_err(|_e| {
            eprintln!(
                "{}",
                r"Failed to open registry 'HKEY_CURRENT_USER\SOFTWARE\Classes\AppUserModelId'."
                    .red()
            );
        })?;
    key.remove_tree("yy-tromb.yy-battery-notifier-rs")
        .inspect_err(|_e| {
            eprintln!(
                "{}",
                r"Failed to remove registry key tree 'HKEY_CURRENT_USER\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs'."
                    .red()
            )
        })?;
    /*key.remove_key("yy-tromb.yy-battery-notifier-rs")
    .inspect_err(|_e| {
        dbg!("ここ");
        eprintln!(
            "{}",
            r"Failed to remove registry key tree 'HKEY_CURRENT_USER\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs'."
                .red()
        )
    })?;
    match CURRENT_USER
        .create(r"SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs")
    {
        Ok(_) =>  anyhow::Result::Err(anyhow::anyhow!(
            "{}",
            r"Failed to remove registry key 'HKEY_CURRENT_USER\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs'. Key surving.".red()
        )),
        Err(e) => {
            if e.code() == windows::core::HRESULT(-2147024894 /*0x80070002*/) {
                anyhow::Result::Ok(())
            } else {
                eprintln!("{}", r"Failed to read registry 'HKEY_CURRENT_USER\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\DisplayName'.".red());
                anyhow::Result::Err(anyhow::Error::from(e))
            }
        }
    }*/
    let key = CURRENT_USER
        .create(r"SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs")
        .inspect_err(|_e| {
            eprintln!("{}", r"Failed to open registry 'HKEY_CURRENT_USER\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs'.".red());
        })?;
    match key.get_string("DisplayName") {
        Ok(v) => {
            return anyhow::Result::Err(anyhow::anyhow!(
                r"Failed to delete registry key 'HKEY_CURRENT_USER\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\DisplayName'. Found: {}",
                v
            ));
        }
        Err(e) => {
            if e.code() == windows::core::HRESULT::from_win32(0x80070002) {
                // do nothing
            } else {
                eprintln!("{} {}", r"Failed to read registry 'HKEY_CURRENT_USER\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\DisplayName' for .".red(),"NOT REASON OF REMOVED".bold().red());
                return anyhow::Result::Err(anyhow::Error::from(e));
            }
        }
    };
    match key.get_string("IconUri") {
        Ok(v) => {
            return anyhow::Result::Err(anyhow::anyhow!(
                r"Failed to delete registry key 'HKEY_CURRENT_USER\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\IconUri'. Found: {}",
                v
            ));
        }
        Err(e) => {
            if e.code() == windows::core::HRESULT::from_win32(0x80070002) {
                // do nothing
            } else {
                eprintln!("{} {}", r"Failed to read registry 'HKEY_CURRENT_USER\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\IconUri' for .".red(),"NOT REASON OF REMOVED".bold().red());
                return anyhow::Result::Err(anyhow::Error::from(e));
            }
        }
    };
    println!("{}", "delete sucuessed!".green().on_black());
    anyhow::Ok(())
}
