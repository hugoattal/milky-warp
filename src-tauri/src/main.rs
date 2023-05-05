// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use enigo::Enigo;
use enigo::MouseControllable;

use screenshots::Screen;

static mut SCREENSHOT_PATH: String = String::new();

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_mouse_location() -> String {
    let enigo = Enigo::new();
    let cursor_location: (i32, i32) = enigo.mouse_location();
    format!("{};{}", cursor_location.0, cursor_location.1)
}

fn capture_screen() -> String {
    let enigo = Enigo::new();
    let cursor_location: (i32, i32) = enigo.mouse_location();
    let screen_index = milky_warp::get_screen_index_from_cursor_pos(cursor_location);

    let display = Screen::all().expect("Couldn't find primary display.");
    let image = display[screen_index as usize].capture().expect("Couldn't capture display.");
    let buffer = image.buffer();

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("screenshot.png");

    fs::write(file_path.clone(), buffer).unwrap();

    file_path.to_str().unwrap().to_owned()
}

#[tauri::command]
fn update_screenshot() -> String {
    unsafe {
        SCREENSHOT_PATH = capture_screen();
        SCREENSHOT_PATH.clone()
    }
}

#[tauri::command]
fn get_screenshot_path() -> String {
    unsafe {
        SCREENSHOT_PATH.clone()
    }
}

fn make_tray() -> tauri::SystemTray {
    let tray_menu = tauri::SystemTrayMenu::new()
        .add_item(tauri::CustomMenuItem::new("quit".to_string(), "Quit"));

    return tauri::SystemTray::new()
        .with_menu(tray_menu);
}

fn handle_tray_event(app: &tauri::AppHandle, event: tauri::SystemTrayEvent) {
    if let tauri::SystemTrayEvent::MenuItemClick { id, .. } = event {
        if id.as_str() == "quit" {
            app.exit(0);
        }
    }
}

fn main() {
    update_screenshot();

    tauri::Builder::default()
        .system_tray(make_tray())
        .on_system_tray_event(handle_tray_event)
        .invoke_handler(tauri::generate_handler![get_mouse_location, get_screenshot_path, update_screenshot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
