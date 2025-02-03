use crate::constants::{HKCU, REG_RUN_PATH};
use std::env;
use winreg::enums::KEY_SET_VALUE;

pub fn is_autoload_enabled() -> bool {
    let run = HKCU
        .open_subkey(REG_RUN_PATH)
        .expect("Failed to open registry key");

    let app_path: std::result::Result<String, std::io::Error> =
        run.get_value(env!("CARGO_PKG_NAME"));

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

    let remove_result = run.delete_value(env!("CARGO_PKG_NAME"));
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

    let set_result = run.set_value(env!("CARGO_PKG_NAME"), &curr_exe.as_os_str());
    match set_result {
        Ok(_) => true,
        Err(err) => {
            eprintln!("Failed to set registry value: {:?}", err);
            return false;
        }
    }
}
