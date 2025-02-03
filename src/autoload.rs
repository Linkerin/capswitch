use crate::constants::{HKCU, REG_RUN_PATH};
use std::env;
use winreg::enums::KEY_SET_VALUE;

pub fn is_autoload_enabled() -> bool {
    let run = HKCU
        .open_subkey(REG_RUN_PATH)
        .expect("Failed to open registry key");

    let record_value: std::result::Result<String, std::io::Error> =
        run.get_value(env!("CARGO_PKG_NAME"));

    let record_value = match record_value {
        Ok(path) => path,
        Err(_) => return false,
    };

    let splitted_value: Vec<&str> = record_value.split(' ').collect();
    let app_path = splitted_value[0];

    let curr_exe = env::current_exe().expect("Failed to get current executable path");

    if app_path != curr_exe.to_str().unwrap() {
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

pub fn set_autoload(flag: Option<String>) -> bool {
    let run = HKCU
        .open_subkey_with_flags(REG_RUN_PATH, KEY_SET_VALUE)
        .expect("Failed to open registry key");

    let curr_exe = env::current_exe().expect("Failed to get current executable path");

    let curr_exe_str = curr_exe
        .to_str()
        .expect("Failed to get current executable path");

    let value = if flag.is_none() {
        curr_exe_str.to_string() // Convert &str to String
    } else {
        format!("{} {}", curr_exe_str, flag.unwrap()) // format! already returns a String
    };

    let set_result = run.set_value(env!("CARGO_PKG_NAME"), &value);
    match set_result {
        Ok(_) => true,
        Err(err) => {
            eprintln!("Failed to set registry value: {:?}", err);
            return false;
        }
    }
}
