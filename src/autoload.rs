use crate::constants::{APP_NAME, HKCU, REG_RUN_PATH};
use std::env;
use winreg::enums::KEY_SET_VALUE;

pub fn is_autoload_enabled() -> bool {
    let run = HKCU
        .open_subkey(REG_RUN_PATH)
        .expect("Failed to open registry key");

    let app_path: std::result::Result<String, std::io::Error> = run.get_value(APP_NAME);

    if app_path.is_err() {
        return false;
    }

    let curr_exe = env::current_exe().expect("Failed to get current executable path");

    if app_path.unwrap() != curr_exe.to_str().unwrap() {
        return false;
    }

    true
}

pub fn remove_autoload() -> bool {
    let run = HKCU
        .open_subkey_with_flags(REG_RUN_PATH, KEY_SET_VALUE)
        .expect("Failed to open registry key");

    let remove_result = run.delete_value(APP_NAME);
    match remove_result {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn set_autoload() -> bool {
    let run = HKCU
        .open_subkey_with_flags(REG_RUN_PATH, KEY_SET_VALUE)
        .expect("Failed to open registry key");

    let curr_exe = env::current_exe().expect("Failed to get current executable path");

    let set_result = run.set_value(APP_NAME, &curr_exe.as_os_str());
    match set_result {
        Ok(_) => true,
        Err(err) => {
            eprintln!("Failed to set registry value: {:?}", err);
            return false;
        }
    }
}

// use std::os::windows::ffi::OsStrExt;
// use windows::{
//     core::PCWSTR,
//     w,
//     Win32::{
//         Foundation,
//         System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED},
//         UI::Shell::{IsUserAnAdmin, ShellExecuteW},
//         UI::WindowsAndMessaging::SHOW_WINDOW_CMD,
//     },
// };

// pub fn elevate_privileges() -> Result<(), Box<dyn std::error::Error>> {
//     // Initialize COM (required for ShellExecuteW)
//     unsafe {
//         CoInitializeEx(None, COINIT_APARTMENTTHREADED)?;
//     }

//     let exe_path = env::current_exe()?;
//     let file: Vec<u16> = exe_path
//         .as_os_str()
//         .encode_wide()
//         .chain(Some(0)) // Add null terminator
//         .collect();

//     // Convert strings to wide (UTF-16) format
//     let verb = w!("runas"); // Verb to request elevation
//                             // let file = w!(curr_exe); // Path to your program
//     let params = w!("--elevated --set-autoload"); // Optional parameters

//     // Use ShellExecuteW to relaunch the program with admin privileges
//     let result = unsafe {
//         ShellExecuteW(
//             Foundation::HWND(0), // No parent window
//             verb,
//             PCWSTR::from_raw(file.as_ptr()),
//             params,
//             PCWSTR::null(), // No working directory
//             SHOW_WINDOW_CMD(1),
//         )
//     };

//     // Check if the elevation succeeded
//     if result.0 <= 32 {
//         eprintln!("Failed to elevate privileges. Error code: {}", result.0);
//         return Err("Failed to elevate privileges".into());
//     }

//     Ok(())
// }

// pub fn is_admin() -> bool {
//     unsafe { IsUserAnAdmin().as_bool() }
// }
