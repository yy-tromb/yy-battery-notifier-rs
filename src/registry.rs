use colored::Colorize;

pub use windows_registry::{CURRENT_USER, LOCAL_MACHINE};
pub enum RegistryValue<'a> {
    String(String),
    Bytes(&'a [u8]),
}

const WIN32_ERROR_E_FILENOTFOUND: windows::core::HRESULT =
    windows::core::HRESULT::from_win32(0x80070002);
const WIN32_ERROR_E_ACCESSDENIED: windows::core::HRESULT =
    windows::core::HRESULT::from_win32(0x80070005);

#[inline]
fn str_to_wide(s: &str) -> windows::core::PCWSTR {
    windows::core::PCWSTR::from_raw(
        s.encode_utf16()
            .chain(std::iter::once(0))
            .collect::<Vec<u16>>()
            .as_ptr(),
    )
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

pub fn check_registered(
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
                    || read_value_info.1 != *value_bytes
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

pub fn delete_tree(
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
                    r"Failed to remove registry tree 'HKEY_LOCAL_MACHINE\{}\{}'.",
                    parent_path, target_tree
                )
                .red()
            );
        } else {
            eprintln!(
                "{}",
                format!(
                    r"Failed to remove registry tree 'HKEY_CURRENT_USER\{}\{}'.",
                    parent_path, target_tree
                )
                .red()
            );
        }
    })?;
    anyhow::Ok(())
}

pub fn delete_values(root: &windows_registry::Key, path: &str, keys: &[&str]) -> anyhow::Result<()> {
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
        tree.remove_value(key).inspect_err(|_e| {
            if root.as_raw() == windows_registry::LOCAL_MACHINE.as_raw() {
                eprintln!(
                    "{}",
                    format!(
                        r"Failed to remove registry key 'HKEY_LOCAL_MACHINE\{}\{}'.",
                        path, key
                    )
                    .red()
                );
            } else {
                eprintln!(
                    "{}",
                    format!(
                        r"Failed to remove registry key 'HKEY_CURRENT_USER\{}\{}'.",
                        path, key
                    )
                    .red()
                );
            }
        })?;
    }
    anyhow::Ok(())
}

pub fn check_deleted(
    root: &windows_registry::Key,
    path: &str,
    keys: &[&str],
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
                if e.code() == WIN32_ERROR_E_FILENOTFOUND {
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
