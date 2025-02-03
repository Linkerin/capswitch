#![windows_subsystem = "windows"]

mod autoload;
mod constants;
mod switch;
mod tray;
mod utils;

pub static mut IS_PAUSED: bool = false;
pub static mut IS_CIRCULAR_SWITCH_MODE: bool = false;

fn main() -> windows::core::Result<()> {
    let _ = utils::check_for_another_instance();
    utils::get_mode();

    tray::create_tray();
    switch::process_switch()?;

    Ok(())
}
