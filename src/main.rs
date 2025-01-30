#![windows_subsystem = "windows"]

use std::mem;
use windows::{
    core::*, Win32::Foundation::*, Win32::UI::Input::KeyboardAndMouse::*,
    Win32::UI::WindowsAndMessaging::*,
};

static mut HOOK: HHOOK = HHOOK(0);

fn main() -> Result<()> {
    unsafe {
        HOOK = SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard_hook_proc), None, 0)?;

        let mut msg = MSG::default();
        while GetMessageA(&mut msg, HWND::default(), 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }

        if !UnhookWindowsHookEx(HOOK).as_bool() {
            return Err(Error::from_win32());
        }
    }
    Ok(())
}

unsafe extern "system" fn keyboard_hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        let kb_struct = &*(lparam.0 as *const KBDLLHOOKSTRUCT);

        if kb_struct.vkCode == u32::from(VK_CAPITAL.0) {
            if wparam.0 == WM_KEYDOWN as usize {
                let shift_state = GetAsyncKeyState(i32::from(VK_SHIFT.0));
                if (shift_state as i16) < 0 {
                    let input = INPUT {
                        r#type: INPUT_KEYBOARD,
                        Anonymous: INPUT_0 {
                            ki: KEYBDINPUT {
                                wVk: VIRTUAL_KEY(VK_CAPITAL.0),
                                wScan: 0,
                                dwFlags: KEYEVENTF_EXTENDEDKEY,
                                time: 0,
                                dwExtraInfo: 0,
                            },
                        },
                    };
                    SendInput(&[input], mem::size_of::<INPUT>() as i32);
                } else {
                    keybd_event(
                        u8::try_from(VK_SHIFT.0).unwrap(),
                        0,
                        KEYBD_EVENT_FLAGS(0),
                        0,
                    );
                    keybd_event(
                        0x12, // VK_MENU (Alt key)
                        0,
                        KEYBD_EVENT_FLAGS(0),
                        0,
                    );
                    keybd_event(0x12, 0, KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0), 0);
                    keybd_event(
                        u8::try_from(VK_SHIFT.0).unwrap(),
                        0,
                        KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0),
                        0,
                    );
                }
                return LRESULT(1);
            }
        }
    }
    CallNextHookEx(HOOK, code, wparam, lparam)
}
