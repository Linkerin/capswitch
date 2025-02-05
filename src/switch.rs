use crate::APP_STATE;
use std::mem;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        UI::{Input::KeyboardAndMouse::*, TextServices::HKL, WindowsAndMessaging::*},
    },
};

static mut HOOK: HHOOK = HHOOK(0);

fn get_foreground_layout() -> HKL {
    unsafe {
        let hwnd: HWND = GetForegroundWindow(); // Get the active window
        let mut process_id: u32 = 0;
        let thread_id = GetWindowThreadProcessId(hwnd, Some(&mut process_id));

        GetKeyboardLayout(thread_id) // Returns the layout for the active window's thread
    }
}

fn change_keyboard_layout(hkl: &HKL) -> LRESULT {
    unsafe {
        let result = SendMessageA(
            GetForegroundWindow(),
            WM_INPUTLANGCHANGEREQUEST,
            WPARAM(0),
            LPARAM(hkl.0),
        );

        result
    }
}

fn create_kbd_input(vk_code: u16, key_up: bool) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(vk_code),
                wScan: 0,
                dwFlags: if key_up {
                    KEYEVENTF_KEYUP
                } else {
                    KEYBD_EVENT_FLAGS(0)
                },
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

fn imitate_keyboard_layout_change() {
    // Presses win + space, then releases space and afterwards win
    let inputs = [
        create_kbd_input(VK_LWIN.0, false),
        create_kbd_input(VK_SPACE.0, false),
        create_kbd_input(VK_SPACE.0, true),
        create_kbd_input(VK_LWIN.0, true),
    ];

    let result = unsafe {
        SendInput(
            &inputs,
            i32::try_from(mem::size_of::<INPUT>()).expect("Converting sizeof Input Failed"),
        )
    };

    if result == 0 {
        panic!("SendInput failed");
    }
}

unsafe extern "system" fn keyboard_hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match APP_STATE.is_paused() {
        Ok(is_paused) => {
            if is_paused {
                return CallNextHookEx(HOOK, code, wparam, lparam);
            }
        }
        Err(e) => {
            eprint!("Error: {:?}", e);
            return CallNextHookEx(HOOK, code, wparam, lparam);
        }
    }

    if code >= 0 {
        let kb_struct = &*(lparam.0 as *const KBDLLHOOKSTRUCT);

        if kb_struct.vkCode == u32::from(VK_CAPITAL.0) {
            if wparam.0 == WM_KEYDOWN as usize {
                let shift_state = GetAsyncKeyState(i32::from(VK_SHIFT.0));
                if (shift_state as i16) < 0 {
                    // shift is pressed
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
                    let cb_size = i32::try_from(mem::size_of::<INPUT>());
                    match cb_size {
                        Ok(cb_size) => {
                            SendInput(&[input], cb_size);
                        }
                        Err(_) => panic!("SendInput failed"),
                    }
                } else {
                    let curr_layout = get_foreground_layout();
                    let is_prev_mode = APP_STATE.is_previous_mode().unwrap_or_else(|e| {
                        eprintln!("Error: {e}");
                        false
                    });

                    if !is_prev_mode {
                        imitate_keyboard_layout_change();
                    } else {
                        let previous_layout = APP_STATE.prev_layout().unwrap_or_else(|e| {
                            eprintln!("Error: {e}");
                            None
                        });
                        match previous_layout {
                            Some(prev_layout) => {
                                if curr_layout == prev_layout {
                                    imitate_keyboard_layout_change();
                                } else {
                                    let result = change_keyboard_layout(&prev_layout);
                                    // In case of success
                                    if result.0 == 0 {
                                        match APP_STATE.set_prev_layout(Some(curr_layout)) {
                                            Ok(_) => {}
                                            Err(err) => {
                                                eprintln!("Didn't manage to set previous layout to state. Error: {}", err)
                                            }
                                        }
                                    }
                                }
                            }
                            None => {
                                match APP_STATE.set_prev_layout(Some(curr_layout)) {
                                    Ok(_) => {}
                                    Err(err) => {
                                        eprintln!("Didn't manage to set previous layout to state. Error: {}", err)
                                    }
                                }
                                imitate_keyboard_layout_change();
                            }
                        }
                    }
                }
                return LRESULT(1);
            }
        }
    }
    CallNextHookEx(HOOK, code, wparam, lparam)
}

pub fn process_switch() -> Result<()> {
    unsafe {
        HOOK = SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard_hook_proc), None, 0)?;

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, HWND::default(), 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }

        if !UnhookWindowsHookEx(HOOK).is_ok() {
            return Err(Error::from_win32());
        }

        Ok(())
    }
}
