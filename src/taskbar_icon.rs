use std::sync::BarrierWaitResult;

use colored::Colorize;
use flume::unbounded;
use tray_icon;

pub enum TaskbarMessage {
    Terminate,
}

pub struct TaskbarIcon {
    tray_icon: tray_icon::TrayIcon,
    tx: flume::Sender<TaskbarMessage>,
    rx: flume::Receiver<TaskbarMessage>,
}

impl TaskbarIcon {
    pub fn new() -> anyhow::Result<Self> {
        use tray_icon::{TrayIconBuilder, menu::Menu};
        let tray_menu = Menu::new();
        {
            use tray_icon::menu::{IsMenuItem, MenuItem, PredefinedMenuItem, Submenu};
            let app_name_item = MenuItem::with_id("app_name", "yy-battery-notifier-rs", true, None);
            let sleep_5min_item = MenuItem::with_id("sleep_5min", "Sleep 5 minutes", true, None);
            let sleep_10min_item = MenuItem::with_id("sleep_10min", "Sleep 10 minutes", true, None);
            let bar_item = PredefinedMenuItem::separator();
            let submenu = Submenu::with_id("modes_submenu", "select modes", true);
            crate::runner::MODE_NAMES
                .get()
                .ok_or_else(|| {
                    anyhow::Error::msg("MODE_NAMES is not initilized. This can not happen.".red())
                })?
                .iter()
                .map(|mode_name| {
                    (Box::new(MenuItem::with_id(
                        format!("mode:{}", mode_name),
                        mode_name,
                        true,
                        None,
                    )) as Box<dyn IsMenuItem>)
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
            .with_menu(Box::new(tray_menu))
            .with_tooltip("yy-battery-notifier-rs")
            .build()
            .unwrap();
        let (tx, rx) = unbounded();
        Ok(TaskbarIcon { tray_icon, tx, rx })
    }

    pub fn run(&self) {
        std::thread::spawn(|| {
            event_handler();
        });
    }
}

fn event_handler() {
    use tray_icon::{TrayIconEvent, menu::MenuEvent};
    loop {
        if let Ok(event) = TrayIconEvent::receiver().try_recv() {
            println!("tray event: {:?}", event);
        }

        if let Ok(event) = MenuEvent::receiver().try_recv() {
            println!("menu event: {:?}", event);
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
