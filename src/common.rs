pub fn error_to_string(error: &anyhow::Error) -> String {
    //ToDo: error handle system
    let msg = format!("{}", error);
    msg.replace("\x1b[31m", "")
        .replace("\x1b[1m", "")
        .replace("\x1b[0m", "")
}

pub fn error_msgbox(error: &anyhow::Error) -> anyhow::Result<()> {
    use windows::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MESSAGEBOX_RESULT, MessageBoxW};
    use windows::core::HSTRING;
    const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
    let title = HSTRING::from(CARGO_PKG_NAME);
    let content = HSTRING::from(error_to_string(error));

    match unsafe { MessageBoxW(None, &content, &title, MB_ICONERROR) } {
        MESSAGEBOX_RESULT(0) => {
            anyhow::Result::Err(anyhow::Error::from(windows::core::Error::from_thread()))
        }
        _ => anyhow::Ok(()),
    }
}
