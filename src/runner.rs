use anyhow::Context as _;
use colored::Colorize;
use flume::unbounded;
use hooq::hooq;
use std::sync::{PoisonError, RwLockWriteGuard};
use std::{
    ops::Deref,
    sync::{Arc, LazyLock, Mutex, OnceLock, RwLock, RwLockReadGuard},
};

pub static MODE_NAMES: OnceLock<Vec<String>> = OnceLock::new();
pub static MODE: LazyLock<Mode> = LazyLock::new(Mode::default);

#[derive(Default, Debug)]
pub struct Mode(RwLock<Option<String>>);

impl Mode {
    pub fn get(&self) -> RwLockReadGuard<'_, Option<String>> {
        self.0.read().unwrap_or_else(|e| e.into_inner())
    }

    pub fn set(&self, value: Option<String>) {
        match self.0.write() {
            Ok(mut guard) => {
                *guard = value;
                drop(guard); // explicit drop to ensure lock is released
            }
            Err(e) => {
                eprintln!(
                    "[{}:{}:{}] mode.lock() in {}::Mode::set\n{}",
                    file!(),
                    line!() - 9, // line of notification_action.lock(),
                    29,          // column of notification_action.lock(),
                    module_path!(),
                    "poison error of RwLock of mode.".red()
                );
                eprintln!("ref: {:?}", e.get_ref());
                let mut guard = e.into_inner();
                *guard = value;
                drop(guard); // explicit drop to ensure lock is released
            }
        }
    }

    pub fn check_write_lock_error(
        &self,
    ) -> Option<PoisonError<RwLockWriteGuard<'_, Option<String>>>> {
        self.0.write().err()
    }
}

/*
PhantomInvariantLifetimeでどうにかできるかも
impl Deref for Mode {
    type Target = RwLockReadGuard<'_, Option<String>>;

    fn deref(&self) -> RwLockReadGuard<'_, Option<String>> {
        self.0.read().unwrap_or_else(|e| e.into_inner())
    }
}
*/

#[derive(Debug)]
pub struct Status<'a> {
    silent: Silent,
    mode_index: Option<usize>,
    mode_list: &'a Vec<String>,
}

impl<'a> Status<'a> {
    pub fn new(
        silent: Silent,
        mode: Option<String>,
        mode_list: &'a Vec<String>,
    ) -> anyhow::Result<Self> {
        if let Some(ref mode) = mode {
            Ok(Self {
                silent,
                mode_index: mode_list.iter().position(|m| m == mode),
                mode_list,
            })
        } else {
            Ok(Self {
                silent,
                mode_index: None,
                mode_list,
            })
        }
    }

    pub fn is_silent(&self) -> bool {
        self.silent.is_silent()
    }

    pub fn get_mode(&self) -> Option<&String> {
        self.mode_index.and_then(|index| self.mode_list.get(index))
    }

    pub fn set_mode_index(&mut self, mode_index: Option<usize>) {
        if let Some(index) = mode_index {
            if self.mode_list.get(index).is_some() {
                self.mode_index = Some(index);
            }
        } else {
            self.mode_index = None;
        }
    }
}

#[derive(Debug)]
pub struct StatusMessage {
    silent_remaining: Option<u64>,
    silent_until: Option<chrono::DateTime<chrono::Local>>,
}

#[derive(Debug)]
pub enum MainMessage {
    ModeChanged(Option<String>),
    StatusChanged(StatusMessage),
    Error(anyhow::Error),
}

#[derive(Debug)]
pub enum ShellMessage {
    Silent5Mins,
    Silent10Mins,
    SilentSpecifiedMins(u64),
    RequireChangeMode,
    ChangeMode(Option<String>),
    Error(anyhow::Error),
}

#[derive(Debug)]
pub struct Silent {
    instant: std::time::Instant,
    to_silent: std::time::Duration,
    pub silent_until: chrono::DateTime<chrono::Local>,
}

impl Silent {
    pub fn new(to_silent_msec: u64) -> Self {
        Self {
            instant: std::time::Instant::now(),
            to_silent: std::time::Duration::from_millis(to_silent_msec),
            silent_until: chrono::Local::now()
                + chrono::Duration::milliseconds(to_silent_msec as i64),
        }
    }

    #[inline]
    pub fn is_silent(&self) -> bool {
        self.instant.elapsed() > self.to_silent
    }

    #[inline]
    pub fn remaining(&self) -> std::time::Duration {
        self.to_silent - self.instant.elapsed()
    }
}

#[derive(Debug)]
pub struct Runner {
    settings: crate::settings::Settings,
    tx_to_main: flume::Sender<ShellMessage>,
    rx_from_shell: flume::Receiver<ShellMessage>,
    tx_to_shell: flume::Sender<MainMessage>,
    rx_from_main: flume::Receiver<MainMessage>,
    silent: Option<Silent>,
}

// auto insert .with_context() between Result (example: func()->Result<>:`func()`) and `?;`
#[hooq(anyhow)]
impl Runner {
    pub fn new(settings: crate::settings::Settings) -> Self {
        let (tx_to_main, rx_from_shell) = flume::unbounded();
        let (tx_to_shell, rx_from_main) = flume::unbounded();
        Self {
            settings,
            tx_to_main,
            rx_from_shell,
            tx_to_shell,
            rx_from_main,
            silent: None,
        }
    }

    #[inline]
    fn start(
        &self,
        notification_action: Arc<Mutex<Option<crate::notification::NotificationAction>>>,
    ) -> anyhow::Result<()> {
        if self.settings.select_mode_when_starts {
            crate::notification::mode_change_notify(
                &self.settings.notification_method,
                Arc::clone(&notification_action),
            )?;
        }

        // temporary
        if self.settings.taskbar_icon {
            crate::taskbar_icon::run(
                self.tx_to_main.clone(),
                self.rx_from_main.clone(),
                Arc::clone(&notification_action),
            )?;
        }

        if let Some(wait_time) = self
            .settings
            .wait_seconds_after_select_mode_notify_when_starts
        {
            std::thread::sleep(std::time::Duration::from_secs(wait_time));
        }

        Ok(())
    }

    pub fn run(self) -> anyhow::Result<()> {
        use crate::notification::{NotificationAction, battery_notify};
        let duration = std::time::Duration::from_secs(self.settings.check_interval);
        let mode_names = MODE_NAMES.get();
        MODE_NAMES
            .set(
                self.settings
                    .modes
                    .keys()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>(),
            )
            .map_err(|_| {
                anyhow::Error::msg(
                    format!(
                        "MODE_NAMES is initialized. Found: {:?} \nThis can not be happen.",
                        mode_names
                    )
                    .red(),
                )
            })?;
        let mode_names = MODE_NAMES.get().ok_or_else(|| {
            anyhow::Error::msg("MODE_NAMES is not initilized. This can not happen.".red())
        })?;
        let mode = &*MODE;
        if let Some(e) = mode.check_write_lock_error() {
            return Err(anyhow::Error::msg(
                format!(
                    "Failed to lock MODE. Unknown poison error!\n Error: {:?}",
                    e
                )
                .red(),
            ));
        }
        mode.set(self.settings.initial_mode.clone());
        let notification_method = &self.settings.notification_method;
        let notification_action: Arc<Mutex<Option<NotificationAction>>> =
            Arc::new(Mutex::new(None));

        self.start(Arc::clone(&notification_action))?;

        // auto insert below expression and .with_context between Result (example:`~~~()`) and `?;`
        // most function return Result<()>, so if to recovery, using Ok(())
        #[hooq::method(
            .or_else(|e| {
            if self.settings.abort_on_error_except_initialize {
                Err(e).$so_far // inserted anyhow::Result::with_context() as above impl Cli
            } else {
                // ToDo: Handle error gracefully
                let path = $path;
                let line = $line;
                let col = $col;
                let expr = hooq::summary!($source);
                eprintln!("[{path}:{line}:{col}]\n{expr}");
                Ok(())
            }
        }))]
        loop {
            let mut action_guard = match notification_action.lock() {
                Ok(action_guard) => action_guard,
                Err(e) => {
                    eprintln!(
                        "[{}:{}:{}] notification_action.lock() in cli::Cli::run\n{}",
                        file!(),
                        line!() - 6, // line of notification_action.lock(),
                        68,          // column of notification_action.lock(),
                        "poison error of Mutex of notification action.".red()
                    );
                    eprintln!("ref: {:?}", e.get_ref());
                    e.into_inner()
                }
            };
            if let Some(action) = &*action_guard {
                match action {
                    NotificationAction::Silent5Mins => {
                        println!("{}", "Silent for 5 minutes action triggered.".yellow());
                        std::thread::sleep(std::time::Duration::from_secs(
                            300u64.saturating_sub(self.settings.check_interval),
                        ));
                    }
                    NotificationAction::Silent10Mins => {
                        println!("{}", "Silent for 10 minutes action triggered.".yellow());
                        std::thread::sleep(std::time::Duration::from_secs(
                            600u64.saturating_sub(self.settings.check_interval),
                        ));
                    }
                    NotificationAction::SilentSpecifiedMins(specified_mins) => {
                        println!(
                            "{}",
                            format!("Silent for {} minutes action triggered.", specified_mins)
                                .yellow()
                        );
                        std::thread::sleep(std::time::Duration::from_secs(
                            specified_mins
                                .saturating_mul(60)
                                .saturating_sub(self.settings.check_interval),
                        ));
                    }
                    NotificationAction::RequireChangeMode => {
                        crate::notification::mode_change_notify(
                            notification_method,
                            Arc::clone(&notification_action),
                        )?;
                        if !self.settings.notify_battery_during_change_mode {
                            // do not clear action for user to miss dismiss change mode notification
                            drop(action_guard);
                            std::thread::sleep(duration);
                            continue;
                        }
                    }
                    NotificationAction::ChangeMode(mode_to_change) => {
                        println!(
                            "{}",
                            format!(r#"change mode to "{:?}" action triggered."#, mode_to_change)
                                .yellow()
                        );

                        if let Some(mode_to_change) = mode_to_change
                            && mode_names.contains(mode_to_change)
                        {
                            mode.set(Some(mode_to_change.clone()));
                        } else if mode_to_change.is_none() {
                            mode.set(None);
                        }
                    }
                    NotificationAction::Error(e) => {
                        Err(anyhow::Error::msg(e.to_string()))?;
                        // cannot convert or deref e to anyhow::Error directly
                        // so must do via String
                    }
                }
                *action_guard = None; // clear action for next check
            }
            drop(action_guard); // Release the lock before checking battery

            {
                println!("mode: {:?}", mode.get().deref());
            }

            #[hooq::method(
                .with_context(||{
                    let path = $path;
                    let line = $line;
                    let col = $col;
                    let expr = hooq::summary!($source);
                    format!("[{path}:{line}:{col}]\n{expr}")
                })
            )]
            let battery_report = match crate::battery::battery_check() {
                Ok(report) => report,
                Err(e) => {
                    if self.settings.abort_on_error_except_initialize {
                        return Err(e);
                    } else {
                        eprintln!("Error on checking battery:\n{:?}", e);
                        std::thread::sleep(duration);
                        continue; // ToDo: Handle error gracefully
                    }
                }
            };
            dbg!(&battery_report);
            self.settings
                .notifications
                .iter()
                .filter(|notification_setting| {
                    crate::notification::judge_notification(notification_setting, &battery_report)
                })
                .try_for_each(|notification_setting| {
                    battery_notify(
                        notification_method,
                        &battery_report,
                        &notification_setting.title,
                        &notification_setting.message,
                        Arc::clone(&notification_action),
                        &notification_setting.input_type,
                        mode_names,
                    )
                })?;
            {
                if let Some(mode) = mode.get().deref()
                    && let Some(mode_setting) = self.settings.modes.get(mode)
                {
                    mode_setting
                        .notifications
                        .iter()
                        .filter(|notification_setting| {
                            crate::notification::judge_notification(
                                notification_setting,
                                &battery_report,
                            )
                        })
                        .try_for_each(|notification_setting| {
                            battery_notify(
                                notification_method,
                                &battery_report,
                                &notification_setting.title,
                                &notification_setting.message,
                                Arc::clone(&notification_action),
                                &notification_setting.input_type,
                                mode_names,
                            )
                        })?;
                }
            }
            println!("check battery and notifying");
            std::thread::sleep(duration);
        }
        #[allow(unreachable_code)]
        Ok(())
    }
}
