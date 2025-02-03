use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

pub const HKCU: RegKey = RegKey::predef(HKEY_CURRENT_USER);

pub const REG_RUN_PATH: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";
