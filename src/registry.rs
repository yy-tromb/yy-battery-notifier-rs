use colored::Colorize;
use windows_registry::LOCAL_MACHINE;

pub fn register() -> anyhow::Result<()> {
    let key = LOCAL_MACHINE
        .create(r"SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs")
        .inspect_err(|_e| {
            eprintln!("{}", r"Failed to open registry 'HKEY_LOCAL_MACHINE\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs'.".red());
        })?;
    key.set_string("DisplayName", "yy-battery-notifier-rs").inspect_err(|_e|{
            eprintln!("{}", r"Failed to set registry 'HKEY_LOCAL_MACHINE\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\DisplayName'.".red());
        })?;
    let icon_uri = std::env::current_exe()?;
    let icon_uri = icon_uri
        .to_str()
        .unwrap_or(r"C:\Program Files\yy-tromb\yy-battery-notifier-rs\\yy-battery-notifier-rs.exe");
    key.set_string("IconUri", icon_uri).inspect_err(|_e|{
            eprintln!("{}", r"Failed to set registry 'HKEY_LOCAL_MACHINE\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\IconUri'.".red());
        })?;
    let reg_display_name = key.get_string("DisplayName").inspect_err(|_e| {
        eprintln!("{}", r"Failed to read registry 'HKEY_LOCAL_MACHINE\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\DisplayName'.".red());
    })?;
    let reg_icon_uri = key.get_string("IconUri").inspect_err(|_e| {
        eprintln!("{}", r"Failed to read registry 'HKEY_LOCAL_MACHINE\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\IconUri'.".red());
    })?;
    if &reg_display_name == "yy-battery-notifier-rs" && reg_icon_uri == icon_uri {
        println!("{}", "register sucuessed!".green().on_black());
        anyhow::Ok(())
    } else {
        anyhow::Result::Err(anyhow::anyhow!("Failed to set correct string to registry."))
    }
}

pub fn delete() -> anyhow::Result<()> {
    let key = LOCAL_MACHINE
        .create(r"SOFTWARE\Classes\AppUserModelId")
        .inspect_err(|_e| {
            eprintln!(
                "{}",
                r"Failed to open registry 'HKEY_LOCAL_MACHINE\SOFTWARE\Classes\AppUserModelId'."
                    .red()
            );
        })?;
    key.remove_tree("yy-tromb.yy-battery-notifier-rs")
        .inspect_err(|_e| {
            eprintln!(
                "{}",
                r"Failed to remove registry key tree 'HKEY_LOCAL_MACHINE\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs'."
                    .red()
            )
        })?;
    /*key.remove_key("yy-tromb.yy-battery-notifier-rs")
    .inspect_err(|_e| {
        dbg!("ここ");
        eprintln!(
            "{}",
            r"Failed to remove registry key tree 'HKEY_LOCAL_MACHINE\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs'."
                .red()
        )
    })?;
    match LOCAL_MACHINE
        .create(r"SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs")
    {
        Ok(_) =>  anyhow::Result::Err(anyhow::anyhow!(
            "{}",
            r"Failed to remove registry key 'HKEY_LOCAL_MACHINE\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs'. Key surving.".red()
        )),
        Err(e) => {
            if e.code() == windows::core::HRESULT(-2147024894 /*0x80070002*/) {
                anyhow::Result::Ok(())
            } else {
                eprintln!("{}", r"Failed to read registry 'HKEY_LOCAL_MACHINE\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\DisplayName'.".red());
                anyhow::Result::Err(anyhow::Error::from(e))
            }
        }
    }*/
    let key = LOCAL_MACHINE
        .create(r"SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs")
        .inspect_err(|_e| {
            eprintln!("{}", r"Failed to open registry 'HKEY_LOCAL_MACHINE\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs'.".red());
        })?;
    match key.get_string("DisplayName") {
        Ok(v) => {
            return anyhow::Result::Err(anyhow::anyhow!(
                r"Failed to remove registry key 'HKEY_LOCAL_MACHINE\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\DisplayName'. Found: {}",
                v
            ));
        }
        Err(e) => {
            if e.code() == windows::core::HRESULT(-2147024894 /*0x80070002*/) {
                // do nothing
            } else {
                eprintln!("{} {}", r"Failed to read registry 'HKEY_LOCAL_MACHINE\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\DisplayName' for .".red(),"NOT REASON OF REMOVED".bold().red());
                return anyhow::Result::Err(anyhow::Error::from(e));
            }
        }
    };
    match key.get_string("IconUri") {
        Ok(v) => {
            return anyhow::Result::Err(anyhow::anyhow!(
                r"Failed to remove registry key 'HKEY_LOCAL_MACHINE\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\IconUri'. Found: {}",
                v
            ));
        }
        Err(e) => {
            if e.code() == windows::core::HRESULT(-2147024894 /*0x80070002*/) {
                // do nothing
            } else {
                eprintln!("{} {}", r"Failed to read registry 'HKEY_LOCAL_MACHINE\SOFTWARE\Classes\AppUserModelId\yy-tromb.yy-battery-notifier-rs\IconUri' for .".red(),"NOT REASON OF REMOVED".bold().red());
                return anyhow::Result::Err(anyhow::Error::from(e));
            }
        }
    };
    println!("{}", "delete sucuessed!".green().on_black());
    anyhow::Ok(())
}
