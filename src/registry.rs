use colored::Colorize;

pub use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

enum RegistryValue {
    String(String),
    Bytes(Vec<u8>),
}

const WINDOWS_REGISTRY_ERROR_FILE_NOT_FOUND: windows::core::HRESULT =
    windows::core::HRESULT::from_win32(0x80070002);

#[inline]
fn str_to_wide(s: &str) -> windows::core::PCWSTR {
    windows::core::PCWSTR::from_raw(
        s.encode_utf16()
            .chain(std::iter::once(0))
            .collect::<Vec<u16>>()
            .as_ptr(),
    )
}

#[inline]
pub fn register_and_check_aumid(root: &windows_registry::Key) -> anyhow::Result<()> {
    let keys_and_values = vec![
        ("DisplayName", RegistryValue::String("yy-battery-notifier-rs".to_string())),
        ("IconUri", RegistryValue::String(
            std::env::current_exe()
                .inspect_err(|_e| eprintln!("{}", "Failed to get current exe".red()))?
                .to_str()
                .unwrap_or(r"C:\Program Files\yy-tromb\yy-battery-notifier-rs\\yy-battery-notifier-rs.exe")
                .to_string()
        )),
    ];
    //register
    register(
        root,
        r"SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs",
        &keys_and_values,
    )?;

    //check
    check_registered(
        root,
        r"SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs",
        &keys_and_values,
    )?;
    println!("{}", "register sucuessed!".green().on_black());
    anyhow::Ok(())
}

#[inline]
pub fn delete_and_check_aumid(root: &windows_registry::Key) -> anyhow::Result<()> {
    let keys = vec!["DisplayName", "IconUri"];
    //delete
    delete(
        root,
        r"SOFTWARE\Classes\AppUserModelId",
        "yy-tromb.yy-battery-notifier-rs",
    )?;
    check_deleted(
        root,
        r"SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs",
        &keys,
    )?;
    println!("{}", "delete sucuessed!".green().on_black());
    anyhow::Ok(())
}

pub fn register(
    root: &windows_registry::Key,
    path: &str,
    keys_and_values: &[(&str, RegistryValue)],
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

fn check_registered(
    root: &windows_registry::Key,
    path: &str,
    keys_and_values: &[(&str, RegistryValue)],
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
                    tree.raw_get_bytes(&key_wide, &mut buffer)
                        .inspect_err(|_e| {
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
                                path, key, value_bytes, &read_value_info.1
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
                                path, key, value_bytes, &read_value_info.1
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

fn delete(
    root: &windows_registry::Key,
    parent_path: &str,
    target_tree: &str,
) -> anyhow::Result<()> {
    let tree = root
        .create(parent_path)
        .inspect_err(|_e| {
            if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                eprintln!("{}", format!(r"Failed to open registry 'HKEY_LOCAL_MACHINE\{}'. You may need to run as administrator.",parent_path).red());
            } else {
                eprintln!("{}", format!(r"Failed to open registry 'HKEY_CURRENT_USER\{}'.",parent_path).red());
            }
        })?;
    tree.remove_tree(target_tree).inspect_err(|_e| {
        if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
            eprintln!(
                "{}",
                format!(
                    r"Failed to remove registry key 'HKEY_LOCAL_MACHINE\{}\{}'.",
                    parent_path, target_tree
                )
                .red()
            );
        } else {
            eprintln!(
                "{}",
                format!(
                    r"Failed to remove registry key 'HKEY_CURRENT_USER\{}\{}'.",
                    parent_path, target_tree
                )
                .red()
            );
        }
    })?;
    anyhow::Ok(())
}

fn check_deleted(root: &windows_registry::Key, path: &str, keys: &[&str]) -> anyhow::Result<()> {
    let tree = root
        .create(path)
        .inspect_err(|_e| {
            if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                eprintln!("{}", format!(r"Failed to open registry 'HKEY_LOCAL_MACHINE\{}'. You may need to run as administrator.",path).red());
            } else {
                eprintln!("{}", format!(r"Failed to open registry 'HKEY_CURRENT_USER\{}'.",path).red());
            }
        })?;
    for key in keys {
        match tree.get_value(key) {
            Ok(v) => {
                if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                    return anyhow::Result::Err(anyhow::anyhow!(
                        "{}",
                        format!(
                            r"Failed to delete registry 'HKEY_LOCAL_MACHINE\{}\{}'.\n Found: {:?}",
                            path, key, v
                        )
                        .red()
                    ));
                } else {
                    return anyhow::Result::Err(anyhow::anyhow!(
                        "{}",
                        format!(
                            r"Failed to read registry 'HKEY_CURRENT_USER\{}\{}'.\n Found: {:?}",
                            path, key, v
                        )
                        .red()
                    ));
                }
            }
            Err(e) => {
                if e.code() == WINDOWS_REGISTRY_ERROR_FILE_NOT_FOUND {
                    // do nothing
                } else {
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
                    return anyhow::Result::Err(anyhow::Error::from(e));
                }
            }
        }
    }
    anyhow::Ok(())
}


