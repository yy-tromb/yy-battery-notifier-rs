use anyhow::Context as _;
use colored::Colorize;
use hooq::hooq;

#[allow(unused)]
use crate::registry::{
    CURRENT_USER, LOCAL_MACHINE, RegistryValue, check_deleted, check_registered, delete_values,
    register,
};

const REG_STARTUP_KEY: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";
const REG_STARTUP_NAME: &str = "yy-tromb.yy-battery-notifier-rs";
const REG_STARTUP_APPROVED_KEY: &str =
    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run";
const REG_STARTUP_APPROVED_VALUE: [u8; 12] = [
    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[hooq(anyhow)]
pub fn register_cli(
    toml_settings_path: Option<String>,
    input_mode: bool,
    default_settings: bool,
) -> anyhow::Result<()> {
    let toml_settings_path: String = match toml_settings_path {
        Some(toml_settings_path) => toml_settings_path,
        None => {
            if (!input_mode) & (!default_settings) {
                return anyhow::Result::Err(anyhow::Error::msg(
                    format!(
                        "This session is {} nor {}.\n\
                    {}. If you want to enable input mode, you can use --input",
                        "input mode".bold(),
                        "using default_settings".bold(),
                        "Use one option".bold()
                    )
                    .red(),
                ));
            } else if input_mode & (!default_settings) {
                let mut toml_settings_path_input = String::with_capacity(256);
                loop {
                    println!(
                        "Input or Paste the path to settings.toml\n\
                    (like C:\\Users\\Documents\\yy-battery-notifier-rs\\settings.toml):"
                    );
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
                toml_settings_path_input
            } else if (!input_mode) & default_settings {
                let default_toml_settings_path = std::env::current_exe()
                    .with_context(|| "Failed to get current exe".red())?
                    .with_file_name("default_settings.toml");
                let default_toml_settings_path =
                    default_toml_settings_path.to_str().with_context(|| {
                        "path to current exe is empty. Unknown error may occured.".red()
                    })?;
                default_toml_settings_path.to_string()
            } else {
                return anyhow::Result::Err(anyhow::Error::msg(
                    format!(
                        "This session is {} and {}.\n\
                    {}. If you want to enable input mode, you can use --input",
                        "input mode".bold(),
                        "using default_settings".bold(),
                        "Use one option".bold()
                    )
                    .red(),
                ));
            }
        }
    };
    let toml_settings_path_absolute = std::fs::canonicalize(toml_settings_path.trim())
        .with_context(|| format!("Failed to canonicalize path: {}", toml_settings_path).red())?;
    let toml_settings_path_absolute = toml_settings_path_absolute
        .to_str()
        .with_context(|| "path to current exe is empty. Unknown error may occured.".red())?;
    println!(
        "Now start register. settings.toml file: '{}'",
        toml_settings_path_absolute //for remove "\\?\" prefix: &toml_settings_path_absolute[4..]
    );
    register_and_check_startup(toml_settings_path_absolute.to_string())?;
    #[cfg(feature = "gui")]
    {
        use windows::Win32::UI::WindowsAndMessaging::{MB_OK, MessageBoxW};
        use windows::core::w;
        unsafe {
            MessageBoxW(
                None,
                w!("register sucuessed!"),
                w!("yy-battery-notifier-rs"),
                MB_OK,
            );
        }
    }
    anyhow::Ok(())
}

#[inline]
#[hooq(anyhow)]
fn register_and_check_startup(toml_settings_path: String) -> anyhow::Result<()> {
    //use windows::ApplicationModel::{StartupTask, StartupTaskState};
    dbg!(&toml_settings_path);

    // registry version
    let current_exe = std::env::current_exe()
        .with_context(|| "Failed to get current exe".red())?
        .with_file_name("yy-battery-notifier-rs_gui.exe");
    let current_exe = current_exe
        .to_str()
        .with_context(|| "path to current exe is empty. Unknown error may occured.".red())?;
    let run_cmd = format!(r#""{}" --msgbox -s "{}""#, current_exe, toml_settings_path);

    //register startup
    let keys_and_values = vec![(REG_STARTUP_NAME, RegistryValue::String(run_cmd))];
    register(CURRENT_USER, REG_STARTUP_KEY, &keys_and_values)?;

    //check startup
    check_registered(CURRENT_USER, REG_STARTUP_KEY, &keys_and_values)?;

    //register approved
    let keys_and_values = vec![(
        REG_STARTUP_NAME,
        RegistryValue::Bytes(&REG_STARTUP_APPROVED_VALUE),
    )];
    register(CURRENT_USER, REG_STARTUP_APPROVED_KEY, &keys_and_values)?;

    //approved check
    check_registered(CURRENT_USER, REG_STARTUP_APPROVED_KEY, &keys_and_values)?;
    println!("{}", "register sucuessed!".green().on_black());
    anyhow::Ok(())
}

#[inline]
#[hooq(anyhow)]
pub fn delete_and_check_startup() -> anyhow::Result<()> {
    //delete startup
    let keys = vec![REG_STARTUP_NAME];
    delete_values(CURRENT_USER, REG_STARTUP_KEY, &keys)?;

    //check deleted startup
    check_deleted(CURRENT_USER, REG_STARTUP_KEY, &keys)?;

    //delete approved
    //let Keys = vec![REG_STARTUP_NAME];
    delete_values(CURRENT_USER, REG_STARTUP_APPROVED_KEY, &keys)?;

    //approved check
    check_deleted(CURRENT_USER, REG_STARTUP_APPROVED_KEY, &keys)?;

    println!("{}", "delete sucuessed!".green().on_black());
    #[cfg(feature = "gui")]
    {
        use windows::Win32::UI::WindowsAndMessaging::{MB_OK, MessageBoxW};
        use windows::core::w;
        unsafe {
            MessageBoxW(
                None,
                w!("delete sucuessed!"),
                w!("yy-battery-notifier-rs"),
                MB_OK,
            );
        }
    }
    anyhow::Ok(())
}
