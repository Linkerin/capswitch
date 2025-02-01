use crate::autoload::{is_autoload_enabled, remove_autoload, set_autoload};
use crate::constants::APP_NAME;
use crate::IS_PAUSED;
use image::ImageReader;
use std::{env, process, thread};
use tray_icon::{
    menu::{
        AboutMetadata, AboutMetadataBuilder, Menu, MenuEvent, MenuId, MenuItem, MenuItemBuilder,
        PredefinedMenuItem,
    },
    Icon, TrayIconBuilder,
};
use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::*};

struct MenuItems {
    toggle: MenuItem,
    mode: MenuItem,
    autoload: MenuItem,
    separator: PredefinedMenuItem,
    about: PredefinedMenuItem,
    quit: MenuItem,
}

fn get_icon() -> Icon {
    let icon_bytes = include_bytes!("../assets/icon.png");
    let icon_img = ImageReader::new(std::io::Cursor::new(icon_bytes))
        .with_guessed_format()
        .expect("Failed to guess image format")
        .decode()
        .expect("Failed to decode image");
    let rgba_bytes = icon_img.to_rgba8().into_raw();
    let icon = Icon::from_rgba(rgba_bytes, icon_img.width(), icon_img.height())
        .expect("Failed to create icon from RGBA bytes");

    return icon;
}

fn get_metadata() -> AboutMetadata {
    let metadata = AboutMetadataBuilder::new()
        .name(Some(env!("CARGO_PKG_NAME")))
        .authors(Some(
            env!("CARGO_PKG_AUTHORS")
                .split(',')
                .map(|s| s.to_string())
                .collect(),
        ))
        .license(Some(env!("CARGO_PKG_LICENSE")))
        .version(Some(env!("CARGO_PKG_VERSION")))
        .website(Some(env!("CARGO_PKG_REPOSITORY")))
        .website_label(Some("GitHub"))
        .comments(Some(env!("CARGO_PKG_DESCRIPTION")))
        .build();

    metadata
}

fn get_menu_items() -> MenuItems {
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

    let metadata = get_metadata();
    let menu_i_about: PredefinedMenuItem = PredefinedMenuItem::about(Some("About"), Some(metadata));

    let items = MenuItems {
        toggle: menu_i_toggle,
        mode: menu_i_mode,
        autoload: menu_i_autoload,
        separator,
        about: menu_i_about,
        quit: menu_i_quit,
    };

    items
}

fn autoload_handler(menu_i: &MenuItem) {
    if is_autoload_enabled() {
        let result = remove_autoload();
        if result {
            menu_i.set_text("Enable autoload");
        }
    } else {
        let result = set_autoload();
        if result {
            menu_i.set_text("Disable autoload");
        }
    }
}

fn mode_hander() {
    println!("Mode menu item clicked");
}

fn quit_hander() {
    println!("Exiting application...");
    process::exit(0);
}

fn toggle_handler(menu_i: &MenuItem) {
    unsafe {
        IS_PAUSED = !IS_PAUSED;

        let text = if IS_PAUSED { "Resume" } else { "Pause" };
        menu_i.set_text(text);
    }
}

pub fn create_tray() {
    thread::spawn(move || {
        let tray_menu: Menu = Menu::new();
        let menu_items: MenuItems = get_menu_items();
        tray_menu
            .append_items(&[
                &menu_items.toggle,
                &menu_items.mode,
                &menu_items.autoload,
                &menu_items.separator,
                &menu_items.about,
                &menu_items.quit,
            ])
            .expect("Failed to add items to tray menu");

        let icon: Icon = get_icon();
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
                        "quit" => quit_hander(),
                        "autoload" => autoload_handler(&menu_items.autoload),
                        "mode" => mode_hander(),
                        "toggle" => toggle_handler(&menu_items.toggle),
                        _ => {
                            println!("Menu item clicked: {:?}", event.id);
                        }
                    }
                }
            }
        }
    });
}
