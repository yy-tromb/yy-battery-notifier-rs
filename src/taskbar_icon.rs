use colored::Colorize;
use std::sync::{Arc, Mutex, OnceLock, RwLock};

#[derive(Debug, Clone)]
pub enum TaskbarMessage {
    Terminate,
    ModeChanged,
}

pub fn run(
    tx_to_main: flume::Sender<crate::runner::ShellMessage>,
    rx_from_main: flume::Receiver<crate::runner::MainMessage>,
    notification_action: Arc<Mutex<Option<crate::notification::NotificationAction>>>,
) -> anyhow::Result<()> {
    use std::thread::spawn;
    spawn(move || {
        let tray_icon = taskbar_icon_init().unwrap();
        spawn(move || {
            channnel_receiver(rx_from_main);
        });
        spawn(tray_icon_event_handler);
        spawn(move || {
            menu_event_handler(Arc::clone(&notification_action));
        });
        win32_event_loop();
    });
    Ok(())
}

fn taskbar_icon_init() -> anyhow::Result<tray_icon::TrayIcon> {
    use tray_controls::{CheckMenuKind, MenuManager};
    use tray_icon::{Icon, TrayIconBuilder, menu::Menu};
    use windows::Win32::UI::HiDpi::{
        DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, SetProcessDpiAwarenessContext,
    };
    match unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) } {
        Ok(_) => {}
        Err(e) => eprintln!(
            "{}{e}",
            "Failed `SetProcessDpiAwarenessContext. Error: `".red()
        ),
    };
    let tray_menu = Menu::new();
    {
        use tray_icon::menu::{IsMenuItem, MenuItem, PredefinedMenuItem, Submenu};

        let app_name_item = MenuItem::with_id("app_name", "yy-battery-notifier-rs", false, None);
        let sleep_5min_item = MenuItem::with_id("sleep_5min", "Sleep 5 minutes", true, None);
        let sleep_10min_item = MenuItem::with_id("sleep_10min", "Sleep 10 minutes", true, None);
        let bar_item = PredefinedMenuItem::separator();
        let submenu = Submenu::with_id("modes_submenu", "select modes", true);
        submenu.append(&MenuItem::with_id("mode_no_mode", "<no mode>", true, None))?;
        crate::runner::MODE_NAMES
            .get()
            .ok_or_else(|| {
                anyhow::Error::msg("MODE_NAMES is not initilized. This can not happen.".red())
            })?
            .iter()
            .map(|mode_name| {
                Box::new(MenuItem::with_id(
                    format!("mode:{}", mode_name),
                    mode_name,
                    true,
                    None,
                )) as Box<dyn IsMenuItem>
            })
            .try_for_each(|item| submenu.append(item.as_ref()))?;
        tray_menu.append_items(&[
            &app_name_item,
            &sleep_5min_item,
            &sleep_10min_item,
            &bar_item,
            &submenu,
        ])?
    }

    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("yy-battery-notifier-rs")
        .with_menu(Box::new(tray_menu))
        .with_menu_on_left_click(true)
        .with_menu_on_right_click(true)
        .with_icon(Icon::from_rgba(
            (0..(32 * 32 * 4)).map(|_| 255).collect(),
            32,
            32,
        )?)
        .build()
        .unwrap();

    Ok(tray_icon)
}

fn channnel_receiver(rx: flume::Receiver<crate::runner::MainMessage>) {
    while let Ok(msg) = rx.recv() {
        println!("{:?}", msg);
        match msg {
            crate::runner::MainMessage::ModeChanged(_) => {}
            crate::runner::MainMessage::SilentChanged(_) => {}
            crate::runner::MainMessage::Error(_) => {}
        }
    }
}

fn tray_icon_event_handler() {
    use tray_icon::TrayIconEvent;

    loop {
        let event = TrayIconEvent::receiver().recv().unwrap();
        println!("tray icon event: {:?}", event);
    }
}

fn menu_event_handler(
    notification_action: Arc<Mutex<Option<crate::notification::NotificationAction>>>,
) {
    use tray_icon::menu::{MenuEvent, MenuId};

    loop {
        let event = MenuEvent::receiver().recv().unwrap();
        println!("menu event: {:?}", event);
        match event {
            MenuEvent { id } if id == MenuId::new("sleep_5min") => {
                let mut guard = match notification_action.lock() {
                    Ok(guard) => guard,
                    Err(e) => e.into_inner(),
                };
                *guard = Some(crate::notification::NotificationAction::Silent5Mins);
            }
            MenuEvent { id } if id == MenuId::new("sleep_10min") => {
                let mut guard = match notification_action.lock() {
                    Ok(guard) => guard,
                    Err(e) => e.into_inner(),
                };
                *guard = Some(crate::notification::NotificationAction::Silent10Mins);
            }
            MenuEvent { id } => {
                if id.0.starts_with("mode:") {
                    let mut guard = match notification_action.lock() {
                        Ok(guard) => guard,
                        Err(e) => e.into_inner(),
                    };
                    *guard = Some(crate::notification::NotificationAction::ChangeMode(Some(
                        id.0[5..].to_string(),
                    )));
                } else if id.0 == "mode_no_mode" {
                    let mut guard = match notification_action.lock() {
                        Ok(guard) => guard,
                        Err(e) => e.into_inner(),
                    };
                    *guard = Some(crate::notification::NotificationAction::ChangeMode(None));
                }
            }
        }
    }
}

fn win32_event_loop() -> anyhow::Error {
    use windows::Win32::UI::WindowsAndMessaging::{
        DispatchMessageW, GetMessageW, MSG, TranslateMessage,
    };
    unsafe {
        let mut msg = MSG::default();
        // このスレッドのキューにあるメッセージ（トレイアイコン用含む）を全部捌く
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        anyhow::Error::from(windows::core::Error::from_thread())
    }
}
