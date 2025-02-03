use crate::IS_CIRCULAR_SWITCH_MODE;
use std::env;
use std::result::Result;
use windows::{
    core::*,
    Win32::{Foundation::*, System::Threading::CreateMutexW, UI::WindowsAndMessaging::*},
};

pub fn check_for_another_instance() -> Result<(), Box<dyn std::error::Error>> {
    // Create a named mutex
    let mutex_name = w!("Global\\CapsWitch");
    let _mutex = unsafe { CreateMutexW(None, true, mutex_name) }?;

    // Check if the mutex already exists (another instance is running)
    let last_error = unsafe { GetLastError() };
    match last_error {
        Ok(_) => {}
        Err(_) => {
            // Show a message box to the user
            unsafe {
                MessageBoxW(
                    None,
                    w!("Another instance of the application is already running."),
                    w!("Application Error"),
                    MB_OK | MB_ICONWARNING,
                );
            }

            std::process::exit(1);
        }
    }

    Ok(())
}

pub fn get_mode() {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1);

    if mode.is_some() && mode.unwrap() == "--circular" {
        unsafe {
            IS_CIRCULAR_SWITCH_MODE = true;
        }
    }
}
