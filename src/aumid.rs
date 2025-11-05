use crate::registry::{check_deleted, check_registered, delete_tree, register, RegistryValue};
use colored::Colorize;

pub const AUMID: &str = "yy-tromb.yy-battery-notifier-rs";

#[inline]
pub fn register_and_check_aumid(root: &windows_registry::Key) -> anyhow::Result<()> {
    let keys_and_values = vec![
        ("DisplayName", RegistryValue::String("yy-battery-notifier-rs".to_string())),
        ("IconUri", RegistryValue::String(
            std::env::current_exe()
                .inspect_err(|_e| eprintln!("{}", "Failed to get current exe".red()))?
                .to_str()
                .unwrap_or(r"C:\Program Files\yy-tromb\yy-battery-notifier-rs\yy-battery-notifier-rs.exe")
                .to_string()
        )),
    ];
    //register
    register(
        root,
        format!(r"SOFTWARE\Classes\AppUserModelId\{}", AUMID).as_str(),
        &keys_and_values,
    )?;

    //check
    check_registered(
        root,
        format!(r"SOFTWARE\Classes\AppUserModelId\{}", AUMID).as_str(),
        &keys_and_values,
    )?;
    println!("{}", "register sucuessed!".green().on_black());
    anyhow::Ok(())
}

#[inline]
pub fn delete_and_check_aumid(root: &windows_registry::Key) -> anyhow::Result<()> {
    let keys = vec!["DisplayName", "IconUri"];
    //delete
    delete_tree(
        root,
        r"SOFTWARE\Classes\AppUserModelId",
        &AUMID,
    )?;
    check_deleted(
        root,
        format!(r"SOFTWARE\Classes\AppUserModelId\{}", AUMID).as_str(),
        &keys,
    )?;
    println!("{}", "delete sucuessed!".green().on_black());
    anyhow::Ok(())
}