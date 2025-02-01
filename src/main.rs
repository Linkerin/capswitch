// #![windows_subsystem = "windows"]

mod autoload;
mod constants;

use crate::autoload::{is_autoload_enabled, remove_autoload, set_autoload};
use crate::constants::APP_NAME;
use image::ImageReader;
use std::{env, mem, process, thread};
use tray_icon::{
    menu::{
        AboutMetadataBuilder, Menu, MenuEvent, MenuId, MenuItem, MenuItemBuilder,
        PredefinedMenuItem,
    },
    Icon, TrayIconBuilder,
};
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
    },
};

static mut IS_PAUSED: bool = false;

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

unsafe extern "system" fn keyboard_hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 && !IS_PAUSED {
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
                            i32::try_from(mem::size_of::<INPUT>())
                                .expect("Converting sizeof Input Failed"),
                        )
                    };

                    if result == 0 {
                        panic!("SendInput failed");
                    }
                }
                return LRESULT(1);
            }
        }
    }
    CallNextHookEx(HOOK, code, wparam, lparam)
}

static mut HOOK: HHOOK = HHOOK(0);

fn main() -> Result<()> {
    // let mut img_path = env::current_dir().expect("Failed to get current directory");
    // img_path.push(r"assets\icon.png");

    thread::spawn(move || {
        // let icon_img = image::open(img_path).expect("Could not load tray icon.");
        // let icon_bytes = icon_img.into_bytes();
        // let icon = Icon::from_rgba(icon_bytes, 256, 256).unwrap();

        let icon_bytes = include_bytes!("../assets/icon.png");
        let icon_img = ImageReader::new(std::io::Cursor::new(icon_bytes))
            .with_guessed_format()
            .expect("Failed to guess image format")
            .decode()
            .expect("Failed to decode image");
        let rgba_bytes = icon_img.to_rgba8().into_raw();
        let icon = Icon::from_rgba(rgba_bytes, icon_img.width(), icon_img.height())
            .expect("Failed to create icon from RGBA bytes");

        let tray_menu: Menu = Menu::new();
        let menu_i_toggle: MenuItem = MenuItemBuilder::new()
            .id(MenuId::new("toggle"))
            .text("Pause")
            .enabled(true)
            .build();
        let menu_i_mode: MenuItem = MenuItemBuilder::new()
            .id(MenuId::new("mode"))
            .text("Circular") // default option will be Previous
            .enabled(false)
            .build();
        let menu_i_autoload: MenuItem = MenuItemBuilder::new()
            .id(MenuId::new("autoload"))
            .text(if is_autoload_enabled() {
                "Disable autoload"
            } else {
                "Enable autoload"
            })
            .enabled(true)
            .build();
        let menu_i_quit: MenuItem = MenuItemBuilder::new()
            .id(MenuId::new("quit"))
            .text("Quit")
            .enabled(true)
            .build();

        let separator = PredefinedMenuItem::separator();
        let metadata = AboutMetadataBuilder::new()
            .name(Some(APP_NAME))
            .authors(Some(vec![String::from("Alexei Gusev")]))
            .license(Some("MIT"))
            .version(Some(env::var("CARGO_PKG_VERSION").unwrap()))
            .build();
        let menu_i_about: PredefinedMenuItem =
            PredefinedMenuItem::about(Some("About"), Some(metadata));

        tray_menu
            .append_items(&[
                &menu_i_toggle,
                &menu_i_mode,
                &menu_i_autoload,
                &separator,
                &menu_i_about,
                &menu_i_quit,
            ])
            .expect("Failed to add items to tray menu");

        let _tray_icon = TrayIconBuilder::new()
            .with_tooltip(APP_NAME)
            .with_icon(icon)
            .with_menu(Box::new(tray_menu))
            .build()
            .unwrap();

        let menu_event_rx = MenuEvent::receiver();

        unsafe {
            let mut msg: MSG = std::mem::zeroed();
            while GetMessageW(&mut msg, HWND(0), 0, 0).into() {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);

                // Check if we received a WM_QUIT message to break the loop
                if msg.message == WM_QUIT {
                    break;
                }

                if let Ok(event) = menu_event_rx.try_recv() {
                    match event.id.as_ref() {
                        "quit" => {
                            println!("Exiting application...");
                            process::exit(0);
                        }
                        "autoload" => {
                            if is_autoload_enabled() {
                                let result = remove_autoload();

                                if result {
                                    menu_i_autoload.set_text("Enable autoload");
                                }
                            } else {
                                let result = set_autoload();

                                if result {
                                    menu_i_autoload.set_text("Disable autoload");
                                }
                            }
                        }
                        "mode" => {
                            println!("Mode menu item clicked");
                        }
                        "toggle" => {
                            IS_PAUSED = !IS_PAUSED;
                            let text = if IS_PAUSED { "Resume" } else { "Pause" };
                            menu_i_toggle.set_text(text);
                        }
                        _ => {
                            println!("Menu item clicked: {:?}", event.id);
                        }
                    }
                }
            }
        }
    });

    unsafe {
        HOOK = SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard_hook_proc), None, 0)?;

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, HWND::default(), 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }

        if !UnhookWindowsHookEx(HOOK).as_bool() {
            return Err(Error::from_win32());
        }
    }

    Ok(())
}

// fn get_thread_id() -> u32 {
//     unsafe {
//         let hwnd: HWND = WindowsAndMessaging::GetForegroundWindow(); // Get the active window
//         let mut process_id: u32 = 0;
//         let thread_id = WindowsAndMessaging::GetWindowThreadProcessId(hwnd, Some(&mut process_id));

//         thread_id
//     }
// }

// fn get_foreground_layout() -> HKL {
//     unsafe {
//         let thread_id = get_thread_id();

//         GetKeyboardLayout(thread_id) // Returns the layout for the active window's thread
//     }
// }
