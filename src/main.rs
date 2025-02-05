#![windows_subsystem = "windows"]

mod autoload;
mod constants;
mod switch;
mod tray;
mod utils;

use std::env;
use std::sync::{LazyLock, RwLock};
use windows::Win32::UI::TextServices::HKL;

#[derive(Debug)]
pub struct AppState {
    _is_paused: RwLock<bool>,
    _is_previous_mode: RwLock<bool>,
    _prev_layout: RwLock<Option<HKL>>,
    _keep_lock: RwLock<bool>,
}

impl AppState {
    fn new(args: Vec<String>) -> Self {
        Self {
            _is_paused: RwLock::new(false),
            _is_previous_mode: RwLock::new(args.get(1).map_or(false, |mode| mode == "--previous")),
            _prev_layout: RwLock::new(None),
            _keep_lock: RwLock::new(false),
        }
    }

    fn is_paused(&self) -> Result<bool, String> {
        let is_paused = *self
            ._is_paused
            .read()
            .map_err(|e| format!("Failed to read `is_paused`: {}", e))?;

        Ok(is_paused)
    }

    fn is_previous_mode(&self) -> Result<bool, String> {
        let is_previous_mode = *self
            ._is_previous_mode
            .read()
            .map_err(|e| format!("Failed to read `is_previous_mode`: {}", e))?;

        Ok(is_previous_mode)
    }

    fn prev_layout(&self) -> Result<Option<HKL>, String> {
        let prev_layout = *self
            ._prev_layout
            .read()
            .map_err(|e| format!("Failed to read `prev_layout`: {}", e))?;

        Ok(prev_layout)
    }

    fn set_prev_layout(&self, layout: Option<HKL>) -> Result<(), String> {
        *self
            ._prev_layout
            .write()
            .map_err(|e| format!("Failed to write `prev_layout`: {}", e))? = layout;

        Ok(())
    }

    fn toggle_pause(&self) -> Result<bool, String> {
        let mut is_paused = self
            ._is_paused
            .write()
            .map_err(|e| format!("Failed to write `is_paused`: {}", e))?;
        let mut _keep_lock = self
            ._keep_lock
            .write()
            .map_err(|e| format!("Failed to write `keep_lock`: {}", e))?;

        *is_paused = !*is_paused;
        drop(is_paused);

        self.is_paused()
    }

    fn toggle_previous_mode(&self) -> Result<bool, String> {
        let mut is_previous_mode = self
            ._is_previous_mode
            .write()
            .map_err(|e| format!("Failed to write `is_previous_mode`: {}", e))?;
        let mut _keep_lock = self
            ._keep_lock
            .write()
            .map_err(|e| format!("Failed to write `keep_lock`: {}", e))?;

        *is_previous_mode = !*is_previous_mode;
        drop(is_previous_mode);

        self.is_previous_mode()
    }
}

pub static APP_STATE: LazyLock<AppState> = LazyLock::new(|| AppState::new(env::args().collect()));

fn main() -> windows::core::Result<()> {
    let _ = utils::check_for_another_instance();
    let _state = &*APP_STATE;

    tray::create_tray();
    switch::process_switch()?;

    Ok(())
}
